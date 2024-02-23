

pub struct MailService {
  pub imap: MailServiceProtocol,
  pub smtp: MailServiceProtocol,
}

pub struct MailServiceDetails {
  pub user: MailServiceUser,
  pub hostname: String,
  pub port: Option<u32>,
  pub ssl: bool,
}

pub struct MailServiceUser {
  pub username: Option<String>,
  pub password: Option<String>,
  pub authentication: MailServiceAuthType,
}


pub enum MailServiceProtocol {
  IMAP,
  SMTP,
}

pub enum MailServiceAuthType {
  Md5,
  Http,
  Ntlm,
  Password,
}