function submitPaste() {
    data = {};

    if(pasteForm.anonymousSwitch.checked) {
        data['author_name'] = 'Anonymous';
    } else {
        if (document.cookie.split(';').filter((item) => item.trim().startsWith('token=')).length) {
            author_name = document.cookie.replace(/(?:(?:^|.*;\s*)username\s*\=\s*([^;]*).*$)|^.*$/, "$1");
            data['author_name'] = author_name;
        } else {
            alert("Login before use real name.");
            return;
        }
    }

    data['content'] = pasteForm.codeText.value;
    data['lang'] = pasteForm.languageSelect.value;
    if(pasteForm.titleInput.value == "") {
        data['title'] = null;
    } else {
        data['title'] = pasteForm.titleInput.value;
    }
    console.log(data);
    var xmlhttp = new XMLHttpRequest(); // new HttpRequest instance
    xmlhttp.open("POST", "/new");
    xmlhttp.setRequestHeader("Content-Type", "application/json;charset=UTF-8");
    xmlhttp.send(JSON.stringify(data));
    xmlhttp.onload  = function() {
        var jsonResponse = JSON.parse(xmlhttp.response);
        window.location.href = "/get/" + jsonResponse['id'];
     };
}

function loginUser() {
    data = {};
    data['username'] = loginForm.username.value;
    data['password'] = loginForm.password.value;
    console.log(data);
    var xmlhttp = new XMLHttpRequest(); // new HttpRequest instance
    xmlhttp.open("POST", "/login");
    xmlhttp.setRequestHeader("Content-Type", "application/json;charset=UTF-8");
    xmlhttp.send(JSON.stringify(data));
    xmlhttp.onload  = function() {
        if (xmlhttp.status == 200) {
            var jsonResponse = JSON.parse(xmlhttp.response);
            document.cookie = "username=" + jsonResponse["user_name"];
            document.cookie = "token=" + jsonResponse["token"];
            document.cookie = "path=/"
            $("#loginModal .close").click();
            location.reload();
        } else {
            loginForm.password.classList.add('is-invalid');
        }
     };
}

function registerUser() {
    data = {};
    data['username'] = registerForm.regusername.value;
    data['password'] = registerForm.regpassword.value;
    console.log(data);
    var xmlhttp = new XMLHttpRequest(); // new HttpRequest instance
    xmlhttp.open("POST", "/register");
    xmlhttp.setRequestHeader("Content-Type", "application/json;charset=UTF-8");
    xmlhttp.send(JSON.stringify(data));
    xmlhttp.onload  = function() {
        if (xmlhttp.status == 200) {
            $("#registerModal .close").click();
        } else {
            registerForm.username.classList.add('is-invalid');
        }
     };
}

function logoutUser() {
    data = {};
    data['token'] = document.cookie.replace(/(?:(?:^|.*;\s*)token\s*\=\s*([^;]*).*$)|^.*$/, "$1");
    var xmlhttp = new XMLHttpRequest(); // new HttpRequest instance
    xmlhttp.open("POST", "/logout");
    xmlhttp.setRequestHeader("Content-Type", "application/json;charset=UTF-8");
    xmlhttp.send(JSON.stringify(data));
    document.cookie.split(";").forEach(function(c) { document.cookie = c.replace(/^ +/, "").replace(/=.*/, "=;expires=" + new Date().toUTCString() + ";path=/"); });
    location.reload()
}

function setNavBar() {
    if (document.cookie.split(';').filter((item) => item.trim().startsWith('token=')).length) {
        linkA.removeAttribute('href');
        linkA.removeAttribute('data-toggle');
        linkA.removeAttribute('data-target');
        linkA.innerText = document.cookie.replace(/(?:(?:^|.*;\s*)username\s*\=\s*([^;]*).*$)|^.*$/, "$1");
        linkB.removeAttribute('href');
        linkB.removeAttribute('data-toggle');
        linkB.removeAttribute('data-target');
        linkB.innerText = "Logout";
        linkB.setAttribute("onclick", "logoutUser()");
        console.log('The cookie "reader" exists (ES6)')
    }
}

window.onload = setNavBar