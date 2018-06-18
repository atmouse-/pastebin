function highlight(language) {
    var type_input = $('#contents_type');
    var code_container = $('pre');
    var worker = new Worker('../hljs_worker.js');
    worker.onmessage = function(event) {
        code_container.html(event.data.value);
        type_input.text(event.data.language);
        $('#contents_type_label').text(event.data.language);
    }
    var message = {text: code_container.text()};
    if (language) {
        message.language = language;
    }
    worker.postMessage(message);
}

function populate_languages() {
    var supported_languages = $('#supported_languages');
    hljs.listLanguages().forEach(function(lang){
        $(supported_languages).append(
            $('<option>', {value: lang, text: lang})
        );
    });
}

$(document).ready(function(){
    populate_languages();

    // Start a highlighting task.
    highlight();
})
