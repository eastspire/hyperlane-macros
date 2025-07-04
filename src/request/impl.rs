use crate::*;

impl Parse for RequestMethods {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(RequestMethods {
            methods: Punctuated::parse_separated_nonempty(input)?,
        })
    }
}

impl Parse for BodyData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let variable: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let type_name: Type = input.parse()?;
        Ok(BodyData {
            variable,
            type_name,
        })
    }
}

impl Parse for AttributeData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key_name: Expr = input.parse()?;
        input.parse::<Token![=>]>()?;
        let variable: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let type_name: Type = input.parse()?;
        Ok(AttributeData {
            variable,
            key_name,
            type_name,
        })
    }
}

impl Parse for AttributesData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let variable: Ident = input.parse()?;
        Ok(AttributesData { variable })
    }
}

impl Parse for RouteParamData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key_name: Expr = input.parse()?;
        input.parse::<Token![=>]>()?;
        let variable: Ident = input.parse()?;
        Ok(RouteParamData { key_name, variable })
    }
}

impl Parse for RouteParamsData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let variable: Ident = input.parse()?;
        Ok(RouteParamsData { variable })
    }
}

impl Parse for QueryData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key_name: Expr = input.parse()?;
        input.parse::<Token![=>]>()?;
        let variable: Ident = input.parse()?;
        Ok(QueryData { key_name, variable })
    }
}

impl Parse for QuerysData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let variable: Ident = input.parse()?;
        Ok(QuerysData { variable })
    }
}

impl Parse for HeaderData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key_name: Expr = input.parse()?;
        input.parse::<Token![=>]>()?;
        let variable: Ident = input.parse()?;
        Ok(HeaderData { key_name, variable })
    }
}

impl Parse for HeadersData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let variable: Ident = input.parse()?;
        Ok(HeadersData { variable })
    }
}
