function submitPaste() {
    if (pasteForm.codeText.value === "") {
        return;
    }
    data = {};

    if (pasteForm.anonymousSwitch.checked) {
        data['author_name'] = 'Anonymous';
    } else {
        if (document.cookie.split(';').filter((item) => item.trim().startsWith('token=')).length) {
            author_name = document.cookie.replace(/(?:(?:^|.*;\s*)username\s*\=\s*([^;]*).*$)|^.*$/, "$1");
            data['author_name'] = author_name;
        } else {
            linkA.click();
            return;
        }
    }

    data['content'] = pasteForm.codeText.value;
    data['lang'] = pasteForm.languageSelect.value;
    if (pasteForm.titleInput.value == "") {
        data['title'] = null;
    } else {
        data['title'] = pasteForm.titleInput.value;
    }
    console.log(data);
    var xmlhttp = new XMLHttpRequest(); // new HttpRequest instance
    xmlhttp.open("POST", "/api/new");
    xmlhttp.setRequestHeader("Content-Type", "application/json;charset=UTF-8");
    xmlhttp.send(JSON.stringify(data));
    xmlhttp.onload = function () {
        var jsonResponse = JSON.parse(xmlhttp.response);
        window.location.href = "/paste/" + jsonResponse['id'];
    };
}

function loginUser() {
    data = {};
    data['username'] = loginForm.username.value;
    data['password'] = loginForm.password.value;
    console.log(data);
    var xmlhttp = new XMLHttpRequest(); // new HttpRequest instance
    xmlhttp.open("POST", "/api/login");
    xmlhttp.setRequestHeader("Content-Type", "application/json;charset=UTF-8");
    xmlhttp.send(JSON.stringify(data));
    xmlhttp.onload = function () {
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
    xmlhttp.open("POST", "/api/register");
    xmlhttp.setRequestHeader("Content-Type", "application/json;charset=UTF-8");
    xmlhttp.send(JSON.stringify(data));
    xmlhttp.onload = function () {
        if (xmlhttp.status == 200) {
            $("#registerModal .close").click();
        } else {
            registerForm.regusername.classList.add('is-invalid');
        }
    };
}

function logoutUser() {
    data = {};
    data['token'] = document.cookie.replace(/(?:(?:^|.*;\s*)token\s*\=\s*([^;]*).*$)|^.*$/, "$1");
    var xmlhttp = new XMLHttpRequest(); // new HttpRequest instance
    xmlhttp.open("POST", "/api/logout");
    xmlhttp.setRequestHeader("Content-Type", "application/json;charset=UTF-8");
    xmlhttp.send(JSON.stringify(data));
    document.cookie.split(";").forEach(function (c) { document.cookie = c.replace(/^ +/, "").replace(/=.*/, "=;expires=" + new Date().toUTCString() + ";path=/"); });
    location.reload()
}

function setNavBar() {
    if (document.cookie.split(';').filter((item) => item.trim().startsWith('token=')).length) {
        linkA.removeAttribute('href');
        linkA.removeAttribute('data-toggle');
        linkA.removeAttribute('data-target');
        linkA.innerText = document.cookie.replace(/(?:(?:^|.*;\s*)username\s*\=\s*([^;]*).*$)|^.*$/, "$1");
        linkA.setAttribute("href", "/user/" + linkA.innerText);
        linkB.removeAttribute('href');
        linkB.removeAttribute('data-toggle');
        linkB.removeAttribute('data-target');
        linkB.innerText = "Logout";
        linkB.setAttribute("onclick", "logoutUser()");
    }
}

function updateSubmitButtonState() {
    if (document.getElementById("codeText").value === "") {
        document.getElementById('submitButton').disabled = true;
    } else {
        document.getElementById('submitButton').disabled = false;
    }
}

$(function () {
    $('#registerModal').keypress(function (e) {
        if (e.which == 13) {
            registerUser();
        }
    })
})


$(function () {
    $('#loginModal').keypress(function (e) {
        if (e.which == 13) {
            loginUser();
        }
    })
})

window.onload = setNavBar
