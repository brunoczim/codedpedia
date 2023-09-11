use url::Url;

pub trait ExternalUrl {
    fn as_url(&self) -> &Url;

    fn into_url(self) -> Url;
}

impl ExternalUrl for Url {
    fn as_url(&self) -> &Url {
        self
    }

    fn into_url(self) -> Url {
        self
    }
}

impl<'url, U> ExternalUrl for &'url U
where
    U: ExternalUrl + ?Sized,
{
    fn as_url(&self) -> &Url {
        (*self).as_url()
    }

    fn into_url(self) -> Url {
        self.as_url().clone()
    }
}
