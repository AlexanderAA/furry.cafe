
use std::borrow::Cow;

use views;
use views::layout::LayoutData;
use models::user::{UserError, User};

pub fn new(errors: Option<UserError>, data: &LayoutData) -> Result<String, ::std::fmt::Error> {
    let mut buffer = String::new();
    let mut partial = String::new();
    try!(html!(partial,
        div.row div class="col-sm-6 offset-sm-3" {
            h1 { "Register" }
            form method="post" action="./" {
                div.form-group {
                    label for="user_name" "Name:"
                    input.form-control type="text" id="user_name" name="user_name" ""
                    @if let &Some(ref errors) = &errors {
                        @for err in &errors.name {
                            p.error.form-text ^err
                        }
                    }
                }
                div.form-group {
                    label for="user_email" "Email:"
                    input.form-control type="text" id="user_email" name="user_email" ""
                    @if let &Some(ref errors) = &errors {
                        @for err in &errors.email {
                            p.error.form-text ^err
                        }
                    }
                }
                div.form-group {
                    label for="user_password" "Password:"
                    input.form-control type="password" id="user_password" name="user_password" ""
                    @if let &Some(ref errors) = &errors {
                        @for err in &errors.password {
                            p.error.form-text ^err
                        }
                    }
                }

                input.btn.btn-primary type="submit" /
            }
        }
    ));

    try!(views::layout::application(&mut buffer, Cow::Borrowed("Register"), Cow::Owned(partial), data));

    Ok(buffer)
}

pub fn index(users: &[User], data: &LayoutData) -> Result<String, ::std::fmt::Error> {
    let mut buffer = String::new();
    let mut partial = String::new();
    try!(html!(partial,
        h1 { "Users" }

        p a href=^(url!("/users/new")) "New User"

        @for user in users {
            div class="user" {
                a.user-link href=^(url!(format!("/users/{}", user.id))) ^user.name
            }
        }
    ));

    try!(views::layout::application(&mut buffer, Cow::Borrowed("Users"), Cow::Owned(partial), data));

    Ok(buffer)
}

pub fn show(user: &User, data: &LayoutData) -> Result<String, ::std::fmt::Error> {
    let mut buffer = String::new();
    let mut partial = String::new();
    try!(html!(partial,
        h1 { ^user.name }

        div.email {
            "Email: "
            ^user.email
        }

        a href=^(url!(format!("/users/{}/edit", user.id))) "Edit"
    ));

    try!(views::layout::application(&mut buffer, Cow::Owned(format!("User: {}", user.name)), Cow::Owned(partial), data));

    Ok(buffer)
}

pub fn edit(user: &User, errors: Option<UserError>, data: &LayoutData) -> Result<String, ::std::fmt::Error> {
    let mut buffer = String::new();
    let mut partial = String::new();
    try!(html!(partial,
        h1 { "Edit User " ^(user.name) }
        form method="post" action=^(format!("/users/{}", user.id)) {
            div.form-group {
                label for="user_name" "Name:"
                input.form-control type="text" id="user_name" name="user_name" value=^user.name /
                @if let &Some(ref errors) = &errors {
                    @for err in &errors.name {
                        p.error.form-text ^err
                    }
                }
            }
            div.form-group {
                label "Email:"
                input.form-control type="text"  disabled="disabled" value=^user.email /
            }
            div.form-group {
                label for="password" "Password:"
                input.form-control type="password" id="password" name="user_password" /
                @if let &Some(ref errors) = &errors {
                    @for err in &errors.password {
                        p.error.form-text ^err
                    }
                }
            }

            input.btn.btn-primary type="submit" /
        }
    ));

    try!(views::layout::application(&mut buffer, Cow::Borrowed("Register"), Cow::Owned(partial), data));

    Ok(buffer)
}

#[cfg(test)]
mod test {
    use models::user::{UserError, User};
    use super::*;

    #[test]
    fn new_user_view_content() {
        let new_string = new(None).unwrap();

        assert!(new_string.contains("user_name"));
        assert!(new_string.contains("user_email"));
        assert!(new_string.contains("user_password"));
    }

    #[test]
    fn new_user_view_errors() {
        let mut errors = UserError::new();
        errors.email.push("Error 1");
        errors.name.push("Error 2");
        errors.password.push("Error 3");
        let new_string = new(Some(errors)).unwrap();

        assert!(new_string.contains("Error 1"));
        assert!(new_string.contains("Error 2"));
        assert!(new_string.contains("Error 3"));
    }

    #[test]
    fn index_user_view() {
        let users = vec![
            User {
                id:            1,
                email:         "test@example.com".into(),
                password_hash: "asdf".into(),
                name:          "Test User #1".into(),
            },
            User {
                id:            2,
                email:         "test2@example.com".into(),
                password_hash: "asdf".into(),
                name:          "Test User #2".into(),
            },
            User {
                id:            3,
                email:         "test3@example.com".into(),
                password_hash: "asdf".into(),
                name:          "Test User #3".into(),
            },
        ];
        let index_string = index(&users).unwrap();
        for user in &users {
            assert!(index_string.contains(&user.name));
            assert!(index_string.contains(&format!("/users/{}", user.id)));
        }
    }

    #[test]
    fn show_user_view() {
        let user = User {
            id:            1,
            email:         "test@example.com".into(),
            password_hash: "asdf".into(),
            name:          "Test User #1".into(),
        };
        let show_string = show(&user).unwrap();
        assert!(show_string.contains(&user.name));
    }

    #[test]
    fn edit_user_view() {
        let user = User {
            id:            1,
            email:         "test@example.com".into(),
            password_hash: "asdf".into(),
            name:          "Test User #1".into(),
        };
        let edit_string = edit(&user, None).unwrap();
        assert!(edit_string.contains("user_name"));
        assert!(edit_string.contains("user_password"));
        assert!(!edit_string.contains("user_email"));
    }

    #[test]
    fn edit_user_view_test() {
        let user = User {
            id:            1,
            email:         "test@example.com".into(),
            password_hash: "asdf".into(),
            name:          "Test User #1".into(),
        };
        let mut errors = UserError::new();
        errors.name.push("Error 1");
        errors.password.push("Error 2");

        let edit_string = edit(&user, Some(errors)).unwrap();
        assert!(edit_string.contains("user_name"));
        assert!(edit_string.contains("user_password"));
        assert!(!edit_string.contains("user_email"));
        assert!(edit_string.contains("Error 1"));
        assert!(edit_string.contains("Error 2"));
    }
}

