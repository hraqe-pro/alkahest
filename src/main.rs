mod server{
    pub mod credentials;
    pub mod server_management {
        pub mod session_manager;
        pub mod action_collection;
    }
    pub mod remote_actions {
        pub mod execute_command;
        pub mod remote_action;
        pub mod manage_packages;
        pub mod upload_file;
        pub mod upload_folder;
    }

    pub mod misc {
        pub mod class;
    }
    pub mod local_actions {
        pub mod local_command;
    }
}

use server::{credentials, remote_actions::execute_command::{ExecuteCommandError, ExecuteCommand}, server_management::{session_manager::{SessionManagerTrait}, action_collection}};
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::{env, error::Error, io::{Read, Write}, path::Path};
use std::ops::DerefMut;
use std::path::PathBuf;
use druid::{Data, widget::{Label, Button}, Env, Widget, WindowDesc, AppLauncher};
use druid::widget::{Flex};

use crate::server::credentials::Credentials;
use crate::server::credentials::ServerFunctionality;
use crate::server::server_management::session_manager::{SessionManager};
use crate::server::remote_actions::remote_action::{Action, ActionEnum};
//use crate::server::server_management::session_manager::execute_command;
//use crate::server::remote_actions::manage_packages::{AddPackage, RemovePackage};

extern crate dotenv;

use dotenv::dotenv;
use std::process::Command;
use ssh2::ErrorCode::Session;
use crate::server::remote_actions::upload_folder::UploadFolder;
use crate::server::remote_actions::upload_file::UploadFile;
use crate::server::server_management::action_collection::{ActionCollection, ActionCollectionTrait};


//widgets////////////////////////////////////////////////////////
#[derive(Clone, Data)]
struct TestWidget {
    num: u32
}
/////////////////////////////////////////////////////////////////

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let credentials = Credentials::new(
        std::env::var("IP").unwrap(),
        std::env::var("USER").unwrap(),
        std::env::var("PORT").unwrap().parse().unwrap(),
        std::env::var("PASSWORD").unwrap(),
    );

    

    let mut sessions = Vec::new();
    let mut collections = Vec::new();

    collections.push(ActionCollection::new());

    match credentials.connect().await {
        Ok(session) => { 
            sessions.push(SessionManager::new(credentials, session));

            println!("connected successfully");
        },
        Err(_) => {
            println!("couldn't connect");
        }
    };


    let test = ExecuteCommand {command: "npm i unzip".to_string(), sudo: false};
    //let test2 = AddPackage {package_name: "unzip".to_string() };



    //collections.last_mut().unwrap().actions.push(ActionEnum::ExecuteCommand(ExecuteCommand{command: "pwd".to_string(), sudo: false}));
    //collections.last_mut().unwrap().actions.push(ActionEnum::UploadFile(UploadFile{ source: PathBuf::from("E:/Projects/Rust/alkahest/TestFolder/File.txt"), destination: PathBuf::from("kkk/kkk.txt") }));

    /*
    collections.last_mut().unwrap().actions.push(ActionEnum::UploadFolder(UploadFolder{
        ip: std::env::var("IP").unwrap(),
        user: std::env::var("USER").unwrap(),
        port: 7502,
        password: std::env::var("PASSWORD").unwrap(),
        source: "E:/Projects/Node/Structural/dyn_assets".to_string(),
        destination: "/var/www/Wstructural/dist/client/assets/".to_string()
    }));
    */


    collections.last_mut().unwrap().actions.push(ActionEnum::UploadFolder(UploadFolder {
        source: "E:/Projects/Rust/alkahest/TestFolder".to_string().into(),
        destination: "/kkk".to_string().into(),
        delete_if_already_exists: true
    }));




    //collections.last_mut().unwrap().actions
    //RemoteActionEnum::ExecuteCommand(ExecuteCommand{command: "ls".to_string(), sudo: false});

    sessions.last().unwrap().execute_collection(collections.last().unwrap());

    //let new_action_collection = ActionCollection { actions: Vec::new(), session_manager_owner: last_session };
    //last_session.action_collections.push(new_action_collection);
    //last_session.action_collections.last_mut().unwrap().actions.push(Box::new(test));
    //last_session.action_collections.last_mut().unwrap().actions.push(Box::new(test2));
   // test_lambdas(last_session, &||last_session.test(&test_str.to_string()));

    Ok(())
}
