# WebUI notes
currently, two html files are provided,  
`admin.html` allows the user to do privileged actions,  
`index.html` is just a read-only status display.  

> the intended implementation is for the websocket server to only listen to localhost and for the admin.html to have a http password, deployment will be done with a bookmark with embedded password to safeguard against deleted cookies or any other issues that may happen due to the more secure implementation.  