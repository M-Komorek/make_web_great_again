const express = require('express')
const app = express()
const port = 8080

const getTransactions = function() {
    const transactions = []
    for (let i = 1; i < 101; i++) {
        transactions.push({
            id: i,
            sender: "Anonymous Sender " + i,
            recipient: "Anonymous Recipient " + i,
            framework: "NodeJS (Express)"
        })
    }

    return transactions
}

const getFib = function(req) {
    let f0 = 0n
    let f1 = 1n
    const start = process.hrtime()
    const number = BigInt(req.query.n)

    for (let i = 0; i <= number; i++) {
        let f2 = f0 + f1
        f0 = f1
        f1 = f2
    }

    const end = process.hrtime(start)

    return {
        result: f0.toString(),
        performance: `${end[0]}s and ${end[1]}ns`
    }
}

app.get('/transactions', (_, res) => {
    res.send(getTransactions())
})

app.get('/fib', (req, res) => {
    res.send(getFib(req))
})

app.listen(port, () => {
    console.log(`Express has launched from localhost:${port}`)
})
