pub const DEFAULT_OP_NAME: &'static str = "__PPGA_INTERNAL_DEFAULT";
pub const ERR_HANDLER_NAME: &'static str = "__PPGA_INTERNAL_HANDLE_ERR";
pub const ERR_CALLBACK_NAME: &'static str = "__PPGA_INTERNAL_DFLT_ERR_CB";
pub const UNPACK_NAME: &'static str = "__PPGA_INTERNAL_UNPACK";
pub const DUMP_NAME: &'static str = "dump";

pub const fn default_op_definition() -> &'static str {
    r#"local function __PPGA_INTERNAL_DEFAULT(x, default) 
    if x ~= nil then return (x) end
    return (default)
end"#
}

pub const fn handle_err_definition() -> &'static str {
    r#"local function __PPGA_INTERNAL_HANDLE_ERR(cb, ...)
    local ok, err = ...
    if err ~= nil then
        ok, err = cb(err)
    end
    return (ok), (err)
end"#
}

pub fn default_err_callback_definition() -> &'static str {
    r#"local function __PPGA_INTERNAL_DFLT_ERR_CB(err)
    error(err)
end"#
}

pub fn unpack_definition() -> &'static str {
    r#"if unpack == nil then
    unpack = table.unpack
end
local function __PPGA_INTERNAL_UNPACK(...)
    return table.unpack({...})
end"#
}

pub fn dump_definition() -> &'static str {
    r#"local function dump(o)
    if type(o) == 'table' then
        local s = ''
        local count = 0
        for k,v in pairs(o) do
            if type(k) ~= 'number' then k = '"'..k..'"' end
            s = s .. '['..k..'] = ' .. dump(v) .. ', '
        end
        return '{ ' .. s .. ' }'
    else
       return tostring(o)
    end
end"#
}

#[derive(Debug, Clone)]
pub struct Snippets {
    default_op_definition: &'static str,
    handle_err_definition: &'static str,
    default_err_callback_definition: &'static str,
    unpack_definition: &'static str,
    dump_definition: &'static str,
}

impl Snippets {
    pub fn new() -> Self {
        Self {
            default_op_definition: default_op_definition(),
            handle_err_definition: handle_err_definition(),
            default_err_callback_definition: default_err_callback_definition(),
            unpack_definition: unpack_definition(),
            dump_definition: dump_definition(),
        }
    }

    pub fn iter<'a>(&'a self) -> SnippetsIter<'a> {
        SnippetsIter { snippets: self }
    }
}

pub struct SnippetsIter<'a> {
    snippets: &'a Snippets,
}

impl<'a> IntoIterator for SnippetsIter<'a> {
    type Item = &'a str;
    type IntoIter = <Vec<&'a str> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            self.snippets.default_op_definition,
            self.snippets.handle_err_definition,
            self.snippets.default_err_callback_definition,
            self.snippets.unpack_definition,
            self.snippets.dump_definition,
        ]
        .into_iter()
    }
}
