const puzzleinput = require("fs").readFileSync("input.txt").toString().split(",")
const inputinstr = 1
 
for (let curr = 0; curr < puzzleinput.length;) {
    
 const [i, num1, num2, num3] = puzzleinput.slice(curr)
 const [p1 = "0", p2 = "0"] = i.split("").reverse().slice(2)
 const thing1 = p1 * 1 ? num1 * 1 : puzzleinput[num1] * 1
 const thing2 = p2 * 1 ? num2 * 1 : puzzleinput[num2] * 1
 const thing3 = num3 * 1
 
  if (i == "99") {
     break
 } 
 
 else if (i.endsWith("1")) {
     puzzleinput[thing3] = String(thing1 + thing2)
     curr += 4
 } 
 
 else if (i.endsWith("2")) {
     puzzleinput[thing3] = String(thing1 * thing2)
     curr += 4
 }  
 
 else if (i === "3") {
     puzzleinput[num1 * 1] = inputinstr
     curr += 2
 }  
 
 else if (i.endsWith("4")) {
     console.log(thing1)
     curr += 2
 } 
}
