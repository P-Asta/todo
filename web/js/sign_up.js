let q = new URLSearchParams("{{param}}");
document.getElementsByName("name")[0].value = q.get("n")
document.getElementsByName("pw")[0].value   = q.get("p")
document.getElementsByName("rpw")[0].value  = q.get("r")
switch ({{error}}){
    case 1: alert("password가 일치하지 않습니다.")
}