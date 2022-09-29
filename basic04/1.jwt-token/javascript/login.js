
const loginButton = document.getElementById('Login');
const username = document.getElementById('inputUsername');
const password = document.getElementById('inputPassword');
const message = document.getElementById("loginMessage");


loginButton.addEventListener("click", () => {
    let xhr = new XMLHttpRequest();
    xhr.open("POST", "/auth/login", true);
    xhr.setRequestHeader("Content-Type", "application/json");

    xhr.onreadystatechange = function () {

        if (xhr.readyState === 4) {
            if (xhr.status === 200) {
                let token = xhr.getResponseHeader("token");
                localStorage.setItem("user-token", token);
                console.log(document.location.origin);
                message.innerText = "Logging Successes";
                window.location.replace(document.location.origin);
            } else {
                console.log(xhr.status);
                console.log(xhr.responseText);
                message.innerText = "Login failed please try again";
            }
        }
    };
    let data = JSON.stringify({
        "username": username.value,
        "password": password.value
    });
    xhr.send(data);
    message.innerText = "Logging ...";
})