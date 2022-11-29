struct user{
    username :string,
    password : string,
    is_active : bool,
    last_login : string,
    email : string,
    id : int,
}
 pub fn __init__(username,password,is_active,last_login,email,id)
{
    let mut username=string::new();
    let mut password=string::new();
    let mut is_active=bool;
    let mut last_login=string::new();
    let mut email=string::new();
    let mut id=int::new();
}
pub fn print_self()
{
    println!("{} {} {} {} {} {}",username,password,is_active,last_login,email,id)
}
pub fn get_last_id()
{
    let mut users = file::read("data/users.csv");
    if users.len()==0
    {
        return 0;
    }
    else 
    {
        return users[users.len()-1].id;
    }

}
pub fn get_user_by_id(id)
{
    let mut users = file::read("data/users.csv");
    for user in users
    {
        if user.id == id
        {
            return user;
        }
    }
    return None;
}
pub fn add(username,email,password,last_login,is_active)
{

}