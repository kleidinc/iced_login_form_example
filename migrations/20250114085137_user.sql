-- Add migration script here
CREATE TABLE "user" (
    user_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    first_name varchar(25) NOT NULL,
    last_name varchar(25) NOT NULL,
    telephone varchar(15) NOT NULL UNIQUE,
    PASSWORD varchar(10)
);
