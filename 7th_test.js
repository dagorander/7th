// All based on: https://www.guru99.com/node-js-create-server-get-data.html

var http = require('http');
var server = http.createServer((function(request,response)
{
    response.writeHead(200,            
    {"Content-Type": "text/plain"});
    response.end("Hello World\n");
}));
server.listen(7000);

var request = require("request"); // installed w/ 'npm install request'
request("http://www.google.com", function(error, response, body)
{
    console.log(body);
});

var express = require('express'); // installed w/ 'npm install express'
var app = express(); // makes an object from the express module!
app.route('/Node').get(function(req,res)
{
    res.send("Tutorial on Node");
});
app.route('/Angular').get(function(req,res)
{
    res.send("Tutorial on Angular");
});
app.get('/',function(req,res) // creates a callback function
{
    res.send('Welcome to Guru99 Tutorials'); // response
});
var server = app.listen(3000,function() {}); //was 'server'

// Then, on to here: https://www.guru99.com/node-js-mongodb.html
// For mongodb stuffs
// I think this all stuff should probably end up on a linode