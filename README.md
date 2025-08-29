i made this because look at the equivalent in powershell:

```powershell
[System.Text.Encoding]::UTF8.GetString([System.Convert]::FromBase64String("YourBase64StringHere"))
```

compared to this:

```
base64 {decode|encode} <string>
```

it's made using the [lib-base64 crate](https://crates.io/crates/lib-base64), which has had some limitations in my tests (or it could be my poor implementation, but I've tried to fix it multiple ways on my end), but it works for my use case.
