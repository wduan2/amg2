const DB_FILE = process.env.DB_FILE || `${require('os').homedir()}/.acct/am2.db`
const sqlite3 = require('sqlite3')
const database = new sqlite3.Database(DB_FILE, () => {
    console.log("create new database connection")
})

const listAllAccount = () => {
    database.serialize(() => {
        database.all('SELECT * FROM acct', (err, rows) => {
            if (err) {
                console.error(err)
            } else {
                rows.forEach(r => console.log(r))
            }
        })
    })    
}

module.exports.listAllAccount = listAllAccount;
