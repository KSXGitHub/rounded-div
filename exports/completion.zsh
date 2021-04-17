#compdef rounded-div

autoload -U is-at-least

_rounded-div() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_rounded-div_commands" \
"*::: :->rounded-div" \
&& ret=0
    case $state in
    (rounded-div)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:rounded-div-command-$line[1]:"
        case $line[1] in
            (u8)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':dividend:_files' \
':divisor:_files' \
&& ret=0
;;
(u16)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':dividend:_files' \
':divisor:_files' \
&& ret=0
;;
(u32)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':dividend:_files' \
':divisor:_files' \
&& ret=0
;;
(u64)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':dividend:_files' \
':divisor:_files' \
&& ret=0
;;
(u128)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':dividend:_files' \
':divisor:_files' \
&& ret=0
;;
(usize)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':dividend:_files' \
':divisor:_files' \
&& ret=0
;;
(i8)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':dividend:_files' \
':divisor:_files' \
&& ret=0
;;
(i16)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':dividend:_files' \
':divisor:_files' \
&& ret=0
;;
(i32)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':dividend:_files' \
':divisor:_files' \
&& ret=0
;;
(i64)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':dividend:_files' \
':divisor:_files' \
&& ret=0
;;
(i128)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':dividend:_files' \
':divisor:_files' \
&& ret=0
;;
(isize)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':dividend:_files' \
':divisor:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
}

(( $+functions[_rounded-div_commands] )) ||
_rounded-div_commands() {
    local commands; commands=(
        "u8:Perform rounded division on two u8 numbers" \
"u16:Perform rounded division on two u16 numbers" \
"u32:Perform rounded division on two u32 numbers" \
"u64:Perform rounded division on two u64 numbers" \
"u128:Perform rounded division on two u128 numbers" \
"usize:Perform rounded division on two usize numbers" \
"i8:Perform rounded division on two i8 numbers" \
"i16:Perform rounded division on two i16 numbers" \
"i32:Perform rounded division on two i32 numbers" \
"i64:Perform rounded division on two i64 numbers" \
"i128:Perform rounded division on two i128 numbers" \
"isize:Perform rounded division on two isize numbers" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'rounded-div commands' commands "$@"
}
(( $+functions[_rounded-div__help_commands] )) ||
_rounded-div__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'rounded-div help commands' commands "$@"
}
(( $+functions[_rounded-div__i128_commands] )) ||
_rounded-div__i128_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'rounded-div i128 commands' commands "$@"
}
(( $+functions[_rounded-div__i16_commands] )) ||
_rounded-div__i16_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'rounded-div i16 commands' commands "$@"
}
(( $+functions[_rounded-div__i32_commands] )) ||
_rounded-div__i32_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'rounded-div i32 commands' commands "$@"
}
(( $+functions[_rounded-div__i64_commands] )) ||
_rounded-div__i64_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'rounded-div i64 commands' commands "$@"
}
(( $+functions[_rounded-div__i8_commands] )) ||
_rounded-div__i8_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'rounded-div i8 commands' commands "$@"
}
(( $+functions[_rounded-div__isize_commands] )) ||
_rounded-div__isize_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'rounded-div isize commands' commands "$@"
}
(( $+functions[_rounded-div__u128_commands] )) ||
_rounded-div__u128_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'rounded-div u128 commands' commands "$@"
}
(( $+functions[_rounded-div__u16_commands] )) ||
_rounded-div__u16_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'rounded-div u16 commands' commands "$@"
}
(( $+functions[_rounded-div__u32_commands] )) ||
_rounded-div__u32_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'rounded-div u32 commands' commands "$@"
}
(( $+functions[_rounded-div__u64_commands] )) ||
_rounded-div__u64_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'rounded-div u64 commands' commands "$@"
}
(( $+functions[_rounded-div__u8_commands] )) ||
_rounded-div__u8_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'rounded-div u8 commands' commands "$@"
}
(( $+functions[_rounded-div__usize_commands] )) ||
_rounded-div__usize_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'rounded-div usize commands' commands "$@"
}

_rounded-div "$@"