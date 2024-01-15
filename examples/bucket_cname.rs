use dotenv;

use xt_oss::oss;
use xt_oss::utils;

#[allow(unused)]
async fn put_cname() {
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let s = r#"-----BEGIN CERTIFICATE-----
MIIF9DCCBNygAwIBAgIQAr+RKfEX6fTDyAKWUi3NYTANBgkqhkiG9w0BAQsFADBu
MQswCQYDVQQGEwJVUzEVMBMGA1UEChMMRGlnaUNlcnQgSW5jMRkwFwYDVQQLExB3
d3cuZGlnaWNlcnQuY29tMS0wKwYDVQQDEyRFbmNyeXB0aW9uIEV2ZXJ5d2hlcmUg
RFYgVExTIENBIC0gRzIwHhcNMjQwMTE0MDAwMDAwWhcNMjQwNDEzMjM1OTU5WjAa
MRgwFgYDVQQDEw9jZG4uazEydHViZS5jb20wggEiMA0GCSqGSIb3DQEBAQUAA4IB
DwAwggEKAoIBAQCwD0eROGdp1E1rtsboAcAuDedneeq+4CA7AfhAofD2Cc12vGJG
6i1SBXbuD4TIrSy+e4ZNQtp2ltrN51gkSSZm51AFwqxvnTTT236KOJLdoyCXNJUr
4xBThJg6MhovGguZqvu6TDnnEaBpj/1/0uynqUPpMAxMGSUOwRaGFezEP1CFpyT6
fOMzmPX8WynMkwUZyzMhQbTkKiDvM6xTIqTa5aaBrCXg16sG832NXYGYFD5jtwC7
Oh0yL9ZXiw1JiWpBq+r7+7P3M4Y6A/qfWbZDzvS5XJPX5dR+zWAmVywkJFlZMSYu
T6rkDIdm8lMnrjAUY0MMdOejYfan1JvnYqvrAgMBAAGjggLgMIIC3DAfBgNVHSME
GDAWgBR435GQX+7erPbFdevVTFVT7yRKtjAdBgNVHQ4EFgQU4IGtzYjMWdLzhK8S
Z3zMjkc4k54wGgYDVR0RBBMwEYIPY2RuLmsxMnR1YmUuY29tMD4GA1UdIAQ3MDUw
MwYGZ4EMAQIBMCkwJwYIKwYBBQUHAgEWG2h0dHA6Ly93d3cuZGlnaWNlcnQuY29t
L0NQUzAOBgNVHQ8BAf8EBAMCBaAwHQYDVR0lBBYwFAYIKwYBBQUHAwEGCCsGAQUF
BwMCMIGABggrBgEFBQcBAQR0MHIwJAYIKwYBBQUHMAGGGGh0dHA6Ly9vY3NwLmRp
Z2ljZXJ0LmNvbTBKBggrBgEFBQcwAoY+aHR0cDovL2NhY2VydHMuZGlnaWNlcnQu
Y29tL0VuY3J5cHRpb25FdmVyeXdoZXJlRFZUTFNDQS1HMi5jcnQwDAYDVR0TAQH/
BAIwADCCAXwGCisGAQQB1nkCBAIEggFsBIIBaAFmAHYA7s3QZNXbGs7FXLedtM0T
ojKHRny87N7DUUhZRnEftZsAAAGNBiXrTwAABAMARzBFAiAbg+gSDNtScMmjh6Z5
J0JuZWdjrxNqldrF/AUMPPUHygIhAPZK18dfD9IzPx0FyQBsk82DjHlL9UhzrCOx
+baEW720AHUASLDja9qmRzQP5WoC+p0w6xxSActW3SyB2bu/qznYhHMAAAGNBiXr
JgAABAMARjBEAiB2E3sVAe+NiiAJWj66StUIGCA+M4BhhZyu2U8ygx6ZJQIgZjIZ
mybht5cqYi9Q0KLxPxlEq6YyGLIxUeZ53K4qgiEAdQDatr9rP7W2Ip+bwrtca+hw
kXFsu1GEhTS9pD0wSNf7qwAAAY0GJesGAAAEAwBGMEQCIAIc5RV4b1frioTRLLca
kpnjyvyYSsb1gIIKLQNAZ55QAiBaZw/eaEGYzpltWj6hTJDx2IwKLQJGcM1/piJ4
QWaf0jANBgkqhkiG9w0BAQsFAAOCAQEAPwDNVb32A5Kqxl5BzEBPi5ml4o9hnC1O
NuNazOROjs+1uXlHAgWnYu3iocQT0PO11TBbVtLtxH1dtIlx99q8HMXnfcXzVRmv
fGyI7eC6SLTsZx5bA4OhRImxDs3d4A2Zs6gSu6rOHlznkqYqnErrSfILamh0pZKd
5GgeQWynjr3lh+RNOS4gGs6WImBYOakfXmunyjkxpZN1rtn5XWj13ntDT5uCSuOt
pc6ggwn/amFZnP3IyISVBTTyqx8rADo7om/dhT1gXMBFlP1wGncJJ3UR4Gkwl4M1
VVkf0+bpLlMFJ8qqszQYrT2f8rSTJJ87FwCNXWnrCbey/UL+AnU7ww==
-----END CERTIFICATE-----
-----BEGIN CERTIFICATE-----
MIIEqjCCA5KgAwIBAgIQDeD/te5iy2EQn2CMnO1e0zANBgkqhkiG9w0BAQsFADBh
MQswCQYDVQQGEwJVUzEVMBMGA1UEChMMRGlnaUNlcnQgSW5jMRkwFwYDVQQLExB3
d3cuZGlnaWNlcnQuY29tMSAwHgYDVQQDExdEaWdpQ2VydCBHbG9iYWwgUm9vdCBH
MjAeFw0xNzExMjcxMjQ2NDBaFw0yNzExMjcxMjQ2NDBaMG4xCzAJBgNVBAYTAlVT
MRUwEwYDVQQKEwxEaWdpQ2VydCBJbmMxGTAXBgNVBAsTEHd3dy5kaWdpY2VydC5j
b20xLTArBgNVBAMTJEVuY3J5cHRpb24gRXZlcnl3aGVyZSBEViBUTFMgQ0EgLSBH
MjCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAO8Uf46i/nr7pkgTDqnE
eSIfCFqvPnUq3aF1tMJ5hh9MnO6Lmt5UdHfBGwC9Si+XjK12cjZgxObsL6Rg1njv
NhAMJ4JunN0JGGRJGSevbJsA3sc68nbPQzuKp5Jc8vpryp2mts38pSCXorPR+sch
QisKA7OSQ1MjcFN0d7tbrceWFNbzgL2csJVQeogOBGSe/KZEIZw6gXLKeFe7mupn
NYJROi2iC11+HuF79iAttMc32Cv6UOxixY/3ZV+LzpLnklFq98XORgwkIJL1HuvP
ha8yvb+W6JislZJL+HLFtidoxmI7Qm3ZyIV66W533DsGFimFJkz3y0GeHWuSVMbI
lfsCAwEAAaOCAU8wggFLMB0GA1UdDgQWBBR435GQX+7erPbFdevVTFVT7yRKtjAf
BgNVHSMEGDAWgBROIlQgGJXm427mD/r6uRLtBhePOTAOBgNVHQ8BAf8EBAMCAYYw
HQYDVR0lBBYwFAYIKwYBBQUHAwEGCCsGAQUFBwMCMBIGA1UdEwEB/wQIMAYBAf8C
AQAwNAYIKwYBBQUHAQEEKDAmMCQGCCsGAQUFBzABhhhodHRwOi8vb2NzcC5kaWdp
Y2VydC5jb20wQgYDVR0fBDswOTA3oDWgM4YxaHR0cDovL2NybDMuZGlnaWNlcnQu
Y29tL0RpZ2lDZXJ0R2xvYmFsUm9vdEcyLmNybDBMBgNVHSAERTBDMDcGCWCGSAGG
/WwBAjAqMCgGCCsGAQUFBwIBFhxodHRwczovL3d3dy5kaWdpY2VydC5jb20vQ1BT
MAgGBmeBDAECATANBgkqhkiG9w0BAQsFAAOCAQEAoBs1eCLKakLtVRPFRjBIJ9LJ
L0s8ZWum8U8/1TMVkQMBn+CPb5xnCD0GSA6L/V0ZFrMNqBirrr5B241OesECvxIi
98bZ90h9+q/X5eMyOD35f8YTaEMpdnQCnawIwiHx06/0BfiTj+b/XQih+mqt3ZXe
xNCJqKexdiB2IWGSKcgahPacWkk/BAQFisKIFYEqHzV974S3FAz/8LIfD58xnsEN
GfzyIDkH3JrwYZ8caPTf6ZX9M1GrISN8HnWTtdNCH2xEajRa/h9ZBXjUyFKQrGk2
n2hcLrfZSbynEC/pSw/ET7H5nWwckjmAJ1l9fcnbqkU/pf6uMQmnfl0JQjJNSg==
-----END CERTIFICATE-----
"#;

    let s2 = r#"-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEAsA9HkThnadRNa7bG6AHALg3nZ3nqvuAgOwH4QKHw9gnNdrxi
RuotUgV27g+EyK0svnuGTULadpbazedYJEkmZudQBcKsb50009t+ijiS3aMglzSV
K+MQU4SYOjIaLxoLmar7ukw55xGgaY/9f9Lsp6lD6TAMTBklDsEWhhXsxD9Qhack
+nzjM5j1/FspzJMFGcszIUG05Cog7zOsUyKk2uWmgawl4NerBvN9jV2BmBQ+Y7cA
uzodMi/WV4sNSYlqQavq+/uz9zOGOgP6n1m2Q870uVyT1+XUfs1gJlcsJCRZWTEm
Lk+q5AyHZvJTJ64wFGNDDHTno2H2p9Sb52Kr6wIDAQABAoIBAFTneXcCAXUa6h84
7OFz/q73C5JuTBOy2Dg8sfDRZ/fvMEPDg415X1O/yOPzXfC8+uf9f/4AcIRiKbDE
kVAmaruTb5bnk+rboF9O6c5UjBcQlSX2OL4KaZQtTJA8Gz59y/xWZcEpA394k6h0
xSaAnJoaWeK5xiRot+NOoMMXDbBxnl0O9Wea/BdjYWcosnuHHFG0skbkW5R/cIY5
DGSHoDpV+v4eATAaeLyIPkidkHtDy3kOKciMTgcA9YEYdF9S+54pO8eXATIFZa6N
371EKrQeHwdRhdxODtr2mk95u5ky6QR8SCxT397rxYxCZ46yKsKEMes3tHStpRgW
D5EhW4kCgYEA6vJCwJ1NiKGcfFfby/p/eYbzrU1fOgP2CMoNq8fTM65RqYuwfale
yMZWDUb+F/EmRt0XX7VkaTgaxjjS3Y2NrqErp7bHmVnwL85/cTTL8kq14U8N08uG
Na0Ol9laPdGLfQc5o7qFHocRKBieMxqf2T8z89VLH40vuidoCO5Vj0cCgYEAv9Yk
OnM639o7+P6HLYKvAFgugjXXO77lbVXba6t9zN3Dd7L64ITQSK9NK1IkqajWwzem
0rXe+x/oyMqFPetPBHX6MTur7cO1eAC/UOS13FXiexNG5En/tQmTIR/g/ENlvWhN
3cQ/emIF4+9TBFkmRV0YRQGTpqm+psJp1tEYOD0CgYEAmTTBMrlhBefdE3h3a9w9
pBHMdQYvQrWyiqi3oXz8zIav2c+tl0QK+wHau22/4/zs6OjjhRXUFIjxbLFBUBgu
e+G9hfA62FU9uu79S+UR2T2+vN3ANoxRSE9BZOPnZMhwHWfIvmuywuVw2qSzBMAn
3JLKXgaIplp69PkdbjV8taECgYBBF0rxnjmizOy+fralRYtVpZYaTmR6fDpij9hE
v0qPIcv6KHuhhM8baofXpCaWfphoYtCy/EztjpUw1C7DpL0J+XHsFKAwq9A+KJrB
uujOG0IempraSRV/ewE2Ixf24HN8AhExpy4Wu33eKk1T01/99ymN+Iv95+itspew
8P+zzQKBgQDBXgVhlg9llbXE/btnP48r8T9lFClaWM9pmZ5UdQi+ZhDJtm2Z9X5Q
L9kGQ9QbrUOd00wjFZFu4i4QI4MFADJcdVgPmJ4QCkV5V3hW3n8cNv+VsfPquRC7
Jk0y1rAW6oYpD00ElFLRv4wUPBADGS2bQoG6idp9SKuW8uJ+gx4T0w==
-----END RSA PRIVATE KEY-----
"#;

    let result = client
        .PutCname()
        .with_domain("cdn.k12tube.com")
        .with_cert_id("12211799-cn-hangzhou")
        .with_certificate(s)
        .with_private_key(s2)
        .with_previous_cert_id("12211799-cn-hangzhou")
        .with_force(true)
        .with_delete_certificate(false)
        .send()
        .await;
    match result {
        Ok(result) => {
            println!("{:#?}", result);
        }
        Err(message) => {
            println!("{:#?}", message);
        }
    }
}

#[allow(unused)]
async fn get_cname_token() {
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let result = client.GetCnameToken("cdn.k12tube.com").await;
    match result {
        Ok(result) => {
            println!("{:#?}", result);
        }
        Err(message) => {
            println!("{:#?}", message);
        }
    }
}

#[allow(unused)]
async fn create_cname_token() {
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let result = client.CreateCnameToken("cdn.k12tube.com").await;
    match result {
        Ok(result) => {
            println!("{:#?}", result);
        }
        Err(message) => {
            println!("{:#?}", message);
        }
    }
}

#[allow(unused)]
/// 获取cname信息
async fn get_cname() {
    let options = utils::options_from_env();
    let client = oss::Client::new(options);

    let result = client.ListCname().await;
    match result {
        Ok(result) => {
            println!("{:#?}", &result.data);
            println!("{}", serde_json::to_string_pretty(&result.data).unwrap());
        }
        Err(message) => {
            println!("{:#?}", message)
        }
    }
}

#[allow(unused)]
/// 获取cname信息
async fn list_cname() {
    let options = utils::options_from_env();
    let client = oss::Client::new(options);

    let result = client.ListCname().await;
    match result {
        Ok(result) => {
            // println!("{:#?}", &result.data);
            println!("{}", serde_json::to_string_pretty(&result.data).unwrap());
        }
        Err(message) => {
            println!("{:#?}", message)
        }
    }
}


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    // get_cname().await;
    // create_cname_token().await;
    // get_cname_token().await;
    put_cname().await;
    // list_cname().await;
    // 74d0f7d6d98b5d02a95e1658b96bed1e
}
