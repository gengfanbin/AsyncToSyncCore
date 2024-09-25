import { syncExec } from './index.js'

function sleepFn(params) {
    console.log("params::",params)
    return new Promise((resolve, reject) => {
        setTimeout(() => {
            resolve("sleepFn end of execution")
        }, 5000)
    })
}

console.log(111)
let res = syncExec(sleepFn.toString(), JSON.stringify(258))
console.log(res)
console.log(222)
