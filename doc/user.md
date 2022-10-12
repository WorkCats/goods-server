# 登录

https://DOMAIN/user/login

`post`

## 请求方式

Content-Type: application/json

## 请求参数

```
{
    auto_login: bool,
    username: String,
    password: String,
}
```

## 返回参数

```
{
    token: String,
    errmsg: String,
    errcode: i8
}
```

# 注册

https://DOMAIN/user/signup

`post`

## 请求方式

Content-Type: application/json

## 请求参数

`authorization:{{TOKEN}}`

```
{
    username: String,
    password: String,
    is_administrator: bool
}
```

## 返回参数

```
{
    errmsg: String,
    errcode: i8
}
```

# 续签

https://DOMAIN/user/autologin

`post`

## 请求方式

Content-Type: application/json

## 请求参数

`authorization:{{TOKEN}}`

## 返回参数

```
{
    errmsg: String,
    errcode: i8
}
```

# 删除对应用户

https://DOMAIN/user/delUser

`post`

## 请求方式

Content-Type: application/json

## 请求参数

`authorization:{{TOKEN}}`

```
{
    username: String
}
```

## 返回参数

```
{
    errmsg: String,
    errcode: i8
}
```

# 搜索对应用户

https://DOMAIN/user/searchUser

`post`

## 请求方式

Content-Type: application/json

## 请求参数

`authorization:{{TOKEN}}`

```
{
    username: String
}
```

## 返回参数

```
{
    user_list: Vec<User>,
    errmsg: String,
    errcode: i8
}
```

# 获取用户列表

https://DOMAIN/user/getUserList

`post`

## 请求方式

Content-Type: application/json

## 请求参数

`authorization:{{TOKEN}}`

## 返回参数

```
{
    user_list: Vec<User>,
    errmsg: String,
    errcode: i8
}
```

# 更新用户

https://DOMAIN/user/updateUse

`post`

## 请求方式

Content-Type: application/json

## 请求参数

`authorization:{{TOKEN}}`

```
{
    username: String,
    password: String,
    is_administrator: bool
}
```

## 返回参数

```
{
    errmsg: String,
    errcode: i8
}
```

# 获取用户名称列表

https://DOMAIN/user/getUsernameList

`post`

## 请求方式

Content-Type: application/json

## 请求参数

`authorization:{{TOKEN}}`

## 返回参数

```
{
    username_list: Vec<String>,
    errmsg: String,
    errcode: i8
}
```

# 获取对应用户的货物列表

https://DOMAIN/user/getGoods

`post`

## 请求方式

Content-Type: application/json

## 请求参数

`authorization:{{TOKEN}}`

```
{
    username: String
}
```

## 返回参数

```
{
    good_list: Vec<Good>,
    errmsg: String,
    errcode: i8
}
```

# 获取是否为管理员权限

https://DOMAIN/user/isAdministrator

`post`

## 请求方式

Content-Type: application/json

## 请求参数

`authorization:{{TOKEN}}`

## 返回参数

```
{
    is_administrator: bool,
    errmsg: String,
    errcode: i8
}
```