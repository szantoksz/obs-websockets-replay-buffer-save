mod modules;

use modules::*;

#[tokio::main]
async fn main() {
    // get exec dir path
    utils::logs::print_log("Getting executable directory path...");
    let exex_dir_path = utils::path::get_exec_dir_path();
    utils::logs::print_log(&format!("Done:\n{}", exex_dir_path.display()));

    // define config file path
    utils::logs::print_log("Defining config file path...");
    let config_file_path = exex_dir_path.join("config.json");
    utils::logs::print_log(&format!("Done:\n{}", config_file_path.display()));

    // drop exec dir path (save memory)
    utils::logs::print_log("Dropping executable directory path...");
    drop(exex_dir_path);
    utils::logs::print_log("Done");

    // verify that the config file exeists
    utils::logs::print_log("Verifying that the config file exists...");
    if !config_file_path.exists() {
        utils::logs::print_log("Done: no");

        // crate a clone of the config file path so the if statement has it's own and the compiler
        // wont complain about dropping it
        let config_file_path = config_file_path.clone();

        // the config file doesn't exist, crate it
        utils::logs::print_log("Crating config file...");
        let config_file = utils::file::create(&config_file_path);
        utils::logs::print_log("Done");

        // drop config file path (save memory)
        utils::logs::print_log("Dropping config file path...");
        drop(config_file_path);
        utils::logs::print_log("Done");

        // give the config file it's default data
        utils::logs::print_log("Writing the default config to the config file...");

        // > get default config
        utils::logs::print_log("  > Getting the default config...");
        let default_config = utils::misc::get_default_config();
        utils::logs::print_log(&format!("  > Done:\n{}", default_config));

        // > write default config to config file
        utils::logs::print_log("  > Writing the default config to the config file...");
        utils::file::write_with_file(&config_file, &default_config);
        utils::logs::print_log("  > Done");

        // default config to config file complete
        utils::logs::print_log("Done");

        // drop the config file (save memory)
        utils::logs::print_log("Dropping the config file...");
        drop(config_file);
        utils::logs::print_log("Done");

        // drop default config (save memory)
        utils::logs::print_log("Dropping the default config...");
        drop(default_config);
        utils::logs::print_log("Done");

        // tell user to configure the config file and run the app again
        println!(
            "\nConfigure the 'config.json' file next to the executable and run the app again."
        );
        utils::misc::safe_quit();
    }
    // the file does exist
    utils::logs::print_log("Done: yes");

    // verify that the config file has data
    utils::logs::print_log("Verifying that the config file has data...");
    if !utils::file::has_data(&config_file_path) {
        utils::logs::print_log("Done: no");

        // crate a clone of the config file path so the if statement has it's own and the compiler
        // wont complain about dropping it
        let config_file_path = config_file_path.clone();

        // give the config file it's default data
        utils::logs::print_log("Wriring the default config to the config file...");

        // > get default config
        utils::logs::print_log("  > Getting the default config...");
        let default_config = utils::misc::get_default_config();
        utils::logs::print_log(&format!("  > Done:\n{}", default_config));

        // > write the default config to the config file
        utils::logs::print_log("  > Writing the default config to the config file...");
        utils::file::write_with_path(&config_file_path, &default_config);
        utils::logs::print_log("  > Done");

        // default config to config file complete
        utils::logs::print_log("Done");

        // drop the config file path (save memory)
        utils::logs::print_log("Dropping the config file path...");
        drop(config_file_path);
        utils::logs::print_log("Done");

        // drop the default config (save memory)
        utils::logs::print_log("Dropping the default config");
        drop(default_config);
        utils::logs::print_log("Done");

        // tell user to configure the config file and run the app again
        println!(
            "\nConfigure the 'config.json' file next to the executable and run the app again."
        );
        utils::misc::safe_quit();
    }
    // the file has data
    utils::logs::print_log("Done: yes");

    // read the config file's data
    utils::logs::print_log("Reading the config file's data...");
    let config_file_data = utils::file::read(&config_file_path);
    utils::logs::print_log(&format!("Done:\n{}", config_file_data));

    // drop the config file path (save memory)
    utils::logs::print_log("Dropping the config file path...");
    drop(config_file_path);
    utils::logs::print_log("Done");

    // load the config file into json
    utils::logs::print_log("Loading the config file's data into json...");
    let config_file_json = utils::json::load(&config_file_data);
    utils::logs::print_log("Done");

    // drop the config file data (save memory)
    utils::logs::print_log("Dropping the config file's data...");
    drop(config_file_data);
    utils::logs::print_log("Done");

    // read obs ws values
    utils::logs::print_log("Reading OBS websocket values from the config file...");

    // clone the config file json for each value so the rust compiler wont complain about the drop
    // that will happen

    // > ip
    utils::logs::print_log("  > Reading ip (obs_ws->ip)...");
    let obs_ws_ip = config_file_json["obs_ws"]["ip"]
        .as_str()
        .unwrap()
        .to_string();
    utils::logs::print_log(&format!("  > Done:\n{}", obs_ws_ip));

    // > port
    utils::logs::print_log("  > Reading port (obs_ws->port)...");
    let obs_ws_port = config_file_json["obs_ws"]["port"].as_u64().unwrap();
    utils::logs::print_log(&format!("  > Done:\n{}", obs_ws_port));

    // > password
    utils::logs::print_log("  > Reading password (obs_ws->passwd)...");
    let obs_ws_passwd = config_file_json["obs_ws"]["passwd"]
        .as_str()
        .unwrap()
        .to_string();
    utils::logs::print_log(&format!("  > Done:\n{}", obs_ws_passwd));

    // obs ws value reading done
    utils::logs::print_log("Done");

    // drop config file json (save memory)
    utils::logs::print_log("Dropping config file json...");
    drop(config_file_json);
    utils::logs::print_log("Done");

    // verify that none of the obs ws values are null and the port is valid
    utils::logs::print_log("Verifying that none of the OBS websocket values are null...");
    if obs_ws_ip.is_empty() || obs_ws_port == 0 || obs_ws_port > 65535 || obs_ws_passwd.is_empty() {
        utils::logs::print_log("Done: yes");
        println!(
            "\nPlease make sure that your config file is configured correctly, follow the comments in the config file if needed, or worst case scenratio delete the config file and run the app again to re-generate it."
        );
        utils::misc::safe_quit();
    }
    // none of them are null
    utils::logs::print_log("Done: no");

    // init obws client
    utils::logs::print_log("Initializing OBS websocket client...");
    let mut obws_client = obws::ws::client_connect(
        &obs_ws_ip,
        u16::try_from(obs_ws_port).unwrap(),
        &obs_ws_passwd,
    )
    .await;
    utils::logs::print_log("Done");

    // drop obs values (save memory)
    utils::logs::print_log("Dropping OBS values...");
    drop(obs_ws_ip);
    #[allow(dropping_copy_types)]
    drop(obs_ws_port);
    drop(obs_ws_passwd);
    utils::logs::print_log("Done");

    // save the reply buffer
    utils::logs::print_log("Saving replay buffer...");
    obws_client.replay_buffer().save().await.unwrap();
    utils::logs::print_log("Done");

    // disconnect from the websocket
    utils::logs::print_log("Disconnecting from the OBS websocket...");
    obws_client.disconnect().await;
    utils::logs::print_log("Done");

    // quit the app
    utils::logs::print_log("Exiting...");
    utils::misc::quit();
}
