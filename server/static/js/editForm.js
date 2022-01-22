$(document).ready(function() {

    $('#input_password1').keyup(function () {
        isFill("#input_password1", "#error_password1", "need a password");
        isSame();
    });


    $('#input_password2').keyup(function () {
        isFill("#input_password2", "#error_password2", "need to confirm password");
        isSame();
    });

    $('#input_email').keyup(function () {
        isEmail($('#input_email'), $('#error_email'));
    });
});

function isSame() {
    isMatch($('#input_password1'), $('#input_password2'), $('#error_password2'));
}


function init_pswd() {
    isFill("#input_password1", "#error_password1", "need a password");
    isFill("#input_password2", "#error_password2", "need to confirm password");
    isSame();
}

function init_email() {
    isEmail($('#input_email'), $('#error_email'));
}

function toogle_email() {
    const email = $('#form_email');
    const password = $('#form_password');
    const title_password = $('#title_password');
    const title_email = $('#title_email');

    if (email.hasClass("inactive")) {
        email.removeClass().addClass("active");
        password.removeClass().addClass("inactive");
        title_email.removeClass().addClass("selected_title");
        title_password.removeClass().addClass("not_selected");
    } else {
        email.removeClass().addClass("inactive");
        title_email.removeClass().addClass("not_selected");
    }

}

function toogle_password() {
    const password = $('#form_password');
    const email = $('#form_email');
    const title_password = $('#title_password');
    const title_email = $('#title_email');

    if (password.hasClass("inactive")) {
        password.removeClass().addClass("active");
        email.removeClass().addClass("inactive")
        title_password.removeClass().addClass("selected_title");
        title_email.removeClass().addClass("not_selected");

    } else {
        password.removeClass().addClass("inactive");
        title_password.removeClass().addClass("not_selected");
    }

}