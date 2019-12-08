function submitPaste() {
    data = {};
    data['author_name'] = 'Anonymous';
    data['content'] = pasteForm.codeText.value;
    data['lang'] = pasteForm.languageSelect.value;
    if(pasteForm.titleInput.value == "") {
        data['title'] = null;
    } else {
        data['title'] = pasteForm.titleInput.value;
    }
    console.log(data);
    var xmlhttp = new XMLHttpRequest(); // new HttpRequest instance
    xmlhttp.open("POST", "../api/new");
    xmlhttp.setRequestHeader("Content-Type", "application/json;charset=UTF-8");
    xmlhttp.send(JSON.stringify(data));
    xmlhttp.onload  = function() {
        var jsonResponse = JSON.parse(xmlhttp.response);
        window.location.href = "../get/" + jsonResponse['id'];
     };
}