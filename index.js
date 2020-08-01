import("./pkg").then(module => {
    module.init();
    
    window.cheater = new module.WordscapesLookupWrapper();

    $("#given_letters, #filter").prop("disabled", false);

    $("#given_letters").on("keyup", function() {
        $("#filter").val('');
        $("#results").html(window.cheater.lookup($(this).val()));
    });

    $("#filter").on("keyup", function() {
        $("#results").html(window.cheater.lookup_filter($("#given_letters").val(), $(this).val()));
    });
});
