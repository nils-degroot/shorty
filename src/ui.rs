use maud::{Markup, DOCTYPE};

pub(crate) async fn ui() -> Markup {
    maud::html! {
        (DOCTYPE)
        head {
            title { "Shorty-UI" }
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            meta name="color-scheme" content="light dark";
            link rel="stylesheet" href="/ui/static/pico.min.css";
            link rel="stylesheet" href="/ui/static/custom.css";
            link rel="icon" href="data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>🐙</text></svg>";
        }
        body {
            header {
                h1 { "Shorty" }
                p { "Shorten URLs and without hassle 🐙." }
            }

            main {
                h2 { "Shorten url" }

                form {
                    fieldset {
                        label {
                            "Url to shorten";

                            input name="url" placeholder="Url";

                            small.text-danger #warnUrlMissing style="display: none;" {
                                "Please enter a url"
                            }
                            small.text-danger #warnUrlInvalid style="display: none;" {
                                "The provided url was invalid"
                            }
                        }
                    }

                    input type="submit" value="Shorten it";
                }

                dialog #successDialog {
                    article {
                        header {
                            p {
                                strong { "Url was shortened successfully" }
                            }
                        }

                        p {
                            span { "Use the following link to visit your url:" }

                            br;

                            a href="http://localhost:3000/ui" target="_blank" #successDialogUrl {
                                "http://localhost:3000/ui"
                            }
                        }

                        footer {
                            button #successDialogClose {
                                "Got it"
                            }
                        }
                    }
                }

                dialog #errorDialog {
                    article {
                        header {
                            p {
                                strong .text-danger { "Oh noo" }
                            }
                        }

                        p { "Something went wrong while shortening your url, please try again later." }

                        footer {
                            button #errorDialogClose {
                                "Alright then"
                            }
                        }
                    }
                }
            }

            script src="/ui/static/main.js" { }
        }
    }
}
