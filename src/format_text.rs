use crate::configuration::Configuration;
use crate::parser::ParseError;

pub fn format_text(file_text: &str, config: &Configuration) -> Result<String, ParseError> {
    let mut chars = file_text.chars();
    let mut formated = String::new();
    crate::parser::parse(config, &mut chars, &mut formated)?;
    Ok(formated)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::DEFAULT_CONFIGURATION;

    #[test]
    fn format_auxiliary() {
        assert_eq!(format_text("", &DEFAULT_CONFIGURATION).unwrap(), "");

        vec![
            ("#", "#"),
            ("#some comment", "# some comment"),
            ("#  some comment\n\n", "# some comment\n\n"),
            ("# \n#some comment\n#", "#\n# some comment\n#"),
        ]
        .into_iter()
        .for_each(|(src, dst)| {
            assert_eq!(format_text(src, &DEFAULT_CONFIGURATION).unwrap(), dst);
            assert_eq!(format_text(dst, &DEFAULT_CONFIGURATION).unwrap(), dst);
        });
    }

    #[test]
    fn format_variables() {
        vec![
            ("-102", "-102"),
            ("var1=-102", "var1 = -102"),
            ("var1='hello'", "var1 = 'hello'"),
            ("x=1+2\ny  +=3  *  4", "x = 1 + 2\ny += 3 * 4"),
            ("stat=item not in [1,2,3]", "stat = item not in [1, 2, 3]"),
            (
                "d=5%3#Yields 2.\nd=5%3#Yields 2.",
                "d = 5 % 3 # Yields 2.\nd = 5 % 3 # Yields 2.",
            ),
        ]
        .into_iter()
        .for_each(|(src, dst)| {
            assert_eq!(format_text(src, &DEFAULT_CONFIGURATION).unwrap(), dst);
            assert_eq!(format_text(dst, &DEFAULT_CONFIGURATION).unwrap(), dst);
        });
    }

    #[test]
    fn format_strings() {
        vec![
            ("''", "''"),
            ("''''''", "''''''"),
            ("''s''s''", "'' s '' s ''"),
            ("'''s'''", "'''s'''"),
            ("'some \\'string\\''", "'some \\'string\\''"),
            ("'#some comment string'", "'#some comment string'"),
            ("'''some \nstring'''", "'''some \nstring'''"),
            ("''/''", "'' / ''"),
            (
                "# String path building
joined='/usr/share'/'projectname'    # => /usr/share/projectname
joined='C:\\foo\\bar'/'D:\\builddir' # => D:/builddir
",
                "# String path building
joined = '/usr/share' / 'projectname' # => /usr/share/projectname
joined = 'C:\\foo\\bar' / 'D:\\builddir' # => D:/builddir
",
            ),
        ]
        .into_iter()
        .for_each(|(src, dst)| {
            assert_eq!(format_text(src, &DEFAULT_CONFIGURATION).unwrap(), dst);
            assert_eq!(format_text(dst, &DEFAULT_CONFIGURATION).unwrap(), dst);
        });
    }

    #[test]
    fn format_arrays() {
        vec![
            ("[]", "[]"),
            ("[[]\n]", "[\n  [],\n]"),
            ("[1,2,true,'abc']", "[1, 2, true, 'abc']"),
            ("[-1,[2,3  ],4 ]", "[-1, [2, 3], 4]"),
            (
                "[[-1  ,  2  ],[3  ,4  ]]+1 + [2]+3+[4]",
                "[[-1, 2], [3, 4]] + 1 + [2] + 3 + [4]",
            ),
            (
                "my_array=[1,2]\nlast_element = my_array [-1]+my_array[1]",
                "my_array = [1, 2]\nlast_element = my_array[-1] + my_array[1]",
            ),
            (
                "my_array+=['something']\n#This also works\nmy_array+='else'",
                "my_array += ['something']\n# This also works\nmy_array += \'else\'",
            ),
            (
                "[\n[-1,2],[[3,4,],\n[5]]]",
                "[\n  [-1, 2],\n  [\n    [3, 4],\n    [5],\n  ],\n]",
            ),
            (
                "[\n[-1,2],#comment\n[3,4],\n#comment\n[5]#comment\n]",
                "[\n  [-1, 2], # comment\n  [3, 4],\n  # comment\n  [5], # comment\n]",
            ),
        ]
        .into_iter()
        .for_each(|(src, dst)| {
            assert_eq!(format_text(src, &DEFAULT_CONFIGURATION).unwrap(), dst);
            assert_eq!(format_text(dst, &DEFAULT_CONFIGURATION).unwrap(), dst);
        });
    }

    #[test]
    fn format_dictionary() {
        vec![
            ("{}", "{}"),
            ("{{}\n,[]}", "{\n  {},\n  [],\n}"),
            ("my_dict={'foo':42,'bar':'baz'}", "my_dict = {'foo': 42, 'bar': 'baz'}"),
            (
                "d={'a'+  'b' :  42}\nk='cd'\nd+={k:43  }  ",
                "d = {'a' + 'b': 42}\nk = 'cd'\nd += {k: 43}",
            ),
        ]
        .into_iter()
        .for_each(|(src, dst)| {
            assert_eq!(format_text(src, &DEFAULT_CONFIGURATION).unwrap(), dst);
            assert_eq!(format_text(dst, &DEFAULT_CONFIGURATION).unwrap(), dst);
        });
    }

    #[test]
    fn format_argument() {
        vec![
            ("()", "()"),
            ("executable('progname','prog.c')", "executable('progname', 'prog.c')"),
            (
                "executable('progname',\nsources: 'prog.c',\nc_args: '-DFOO=1')",
                "executable(\n  'progname',\n  sources: 'prog.c',\n  c_args: '-DFOO=1',\n)",
            ),
            (
                "  executable('progname',kwargs: d)",
                "executable('progname', kwargs: d)",
            ),
            (
                "executable('name',['main.c','lib.c'])",
                "executable('name', ['main.c', 'lib.c'])",
            ),
            (
                "executable('name',\n['main.c','lib.c'])",
                "executable(\n  'name',\n  ['main.c', 'lib.c'],\n)",
            ),
            (
                "executable('name',['main.c',\n'lib.c'])",
                "executable('name', [\n  'main.c',\n  'lib.c',\n])",
            ),
            (
                "myobj=some_function()\nmyobj.do_something('now')",
                "myobj = some_function()\nmyobj.do_something('now')",
            ),
        ]
        .into_iter()
        .for_each(|(src, dst)| {
            assert_eq!(format_text(src, &DEFAULT_CONFIGURATION).unwrap(), dst);
            assert_eq!(format_text(dst, &DEFAULT_CONFIGURATION).unwrap(), dst);
        });
    }

    #[test]
    fn format_statement_if() {
        vec![
            ("if a==b\n#do something \nendif", "if a == b\n  # do something\nendif"),
            (
                "if item  not in  list\n#do something\nendif",
                "if item not in list\n  # do something\nendif",
            ),
            (
                "if item not in [1,2,3]\n#do something\nendif",
                "if item not in [1, 2, 3]\n  # do something\nendif",
            ),
            (
                "
var1 = 1
var2 = 2
if var1 == var2 # Evaluates to false
something_broke()
elif var3 == var2
something_else_broke()
else
everything_ok()
endif

opt = get_option('someoption')
if opt != 'foo'
do_something()
endif

if a and b
# do something
if not e
  # do something
endif
if not (f   or g)
# do something
endif
endif
",
                "
var1 = 1
var2 = 2
if var1 == var2 # Evaluates to false
  something_broke()
elif var3 == var2
  something_else_broke()
else
  everything_ok()
endif

opt = get_option('someoption')
if opt != 'foo'
  do_something()
endif

if a and b
  # do something
  if not e
    # do something
  endif
  if not(f or g)
    # do something
  endif
endif
",
            ),
        ]
        .into_iter()
        .for_each(|(src, dst)| {
            assert_eq!(format_text(src, &DEFAULT_CONFIGURATION).unwrap(), dst);
            assert_eq!(format_text(dst, &DEFAULT_CONFIGURATION).unwrap(), dst);
        });
    }

    #[test]
    fn format_statement_foreach() {
        vec![
            (
                "
foreach item:items
#do something
#do something
endforeach",
                "
foreach item : items
  # do something
  # do something
endforeach",
            ),
            (
                "
items=['a','continue','b','break','c']
result=[]
foreach i : items
if i=='continue'
continue
elif i=='break'
break
endif
result += i
endforeach
# result is ['a', 'b']",
                "
items = ['a', 'continue', 'b', 'break', 'c']
result = []
foreach i : items
  if i == 'continue'
    continue
  elif i == 'break'
    break
  endif
  result += i
endforeach
# result is ['a', 'b']",
            ),
        ]
        .into_iter()
        .for_each(|(src, dst)| {
            assert_eq!(format_text(src, &DEFAULT_CONFIGURATION).unwrap(), dst);
            assert_eq!(format_text(dst, &DEFAULT_CONFIGURATION).unwrap(), dst);
        });
    }
}
