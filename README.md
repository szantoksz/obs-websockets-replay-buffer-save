# OBS websockets replay buffer save
An app that talks to the OBS WebSocket Server to save the replay buffer.

## Usage
- 1st: Download the executable (```obs-websockets-replay-buffer-save```) and run it from the terminal, it will create a ```config.json``` file that you'll have to configure.
- 2nd: If you configured the config file correctly you can just run the executable whenever you want and it will save the replay buffer.
### NOTE: OBS with the WebSocket server has to be running for the app to be able to communicate with it, and you need to start the replay buffer in OBS too.


## Configuring the config file
Once you run the app, it will create a ```config.json``` file that will look like this:
```
{
  "conf_file_meta": {
    "ver": "1.0.0"
  },
  "obs_ws": {
    "ip": null,
    "_comment_ip": "The IP address of the OBS websocket, example: \"127.0.0.1\" (Ignore the backslashes)",
    "port": null,
    "_comment_port": "The port of the OBS websocket, example: 4455",
    "passwd": null,
    "_comment_passwd": "The password of the obs websocket, example: \"12345678\" (Ignore the backslashes)"
  }
}
```
The configuration is really simple, you basically have to follow the comments, but here are some examples for each:
 - ip: ```"127.0.0.1"``` or ```"localhost"```
 - port: ```4455``` or any other port that you configure in the OBS WebSocket Server
 - passwd: ```"12345678"``` or any other password you configure in the OBS WebSocket Server

 ### NOTE: Make sure the put the ```ip``` and ```passwd``` in double quotes: ```"<value>"```

## Why did I make this?
I made this because wayland the display server protocol of my choice on linux doesn't have global hotkey support unlike X11, but it does allow for the Desktop Environment to make its own hotkeys and also to run an app on the hotkey. So in that way I can use a hotkey to save clips.

---
## Issues/questions
If you encounter any issues/questions, please leave a comment on the repo's issues tab.

## Contributing

 If you want to help out, feel free to do it with pull requests!

---

## License

 Everything in this repo is licensed under MIT License.
