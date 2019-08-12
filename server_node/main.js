const express = require('express');
const { Client } = require('pg')

const app = express();
const port = 8000;

const client = new Client();
client.connect()
client.query('SELECT $1::text as message', ['Hello world!'], (err, res) => {
    console.log(err ? err.stack : res.rows[0].message) // Hello World!
    client.end()
})

app.use(function(req, res, next) {
    res.header('Access-Control-Allow-Origin', 'http://localhost:4200'); // update to match the domain you will make the request from
    res.header('Access-Control-Allow-Headers', 'Origin, X-Requested-With, Content-Type, Accept');
    next();
});

app.get('/games', (req, res) => {
    console.log(`Received: ${req}`);
    res.json({
        games: [{
            game_id: 'snake',
            name: 'Snake'
        }]
    });
});

app.listen(port, () => console.log(`Example app listening on port ${port}!`));
