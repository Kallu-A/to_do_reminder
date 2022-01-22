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
    $('#form_password').removeClass().addClass("inactive");
    $('#title_password').removeClass()
    email.toggle(function () {
        $('#title_email').removeClass().addClass("selected_title");
        email.removeClass().addClass("active");
    }, function () {
        email.removeClass().addClass("inactive");
        $('#title_email').removeClass();
    });


}

function toogle_password() {
    const password = $('#form_password');
    $('#form_email').removeClass().addClass("inactive");
    $('#title_email').removeClass()
    password.toggle(function () {
        password.removeClass().addClass("active");
        $('#title_password').removeClass().addClass("selected_title");
    }, function () {
        password.removeClass().addClass("inactive");
        $('#title_password').removeClass()
    });

}