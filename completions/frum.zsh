#compdef frum

autoload -U is-at-least

_frum() {
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
'--log-level=[The log level of frum commands \[default: info\] \[possible values: quiet, info, error\]]' \
'--ruby-build-mirror=[\[default: https://cache.ruby-lang.org/pub/ruby\]]' \
'--frum-dir=[The root directory of frum installations \[default: $HOME/.frum\]]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_frum_commands" \
"*::: :->frum" \
&& ret=0
    case $state in
    (frum)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:frum-command-$line[1]:"
        case $line[1] in
            (init)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
(install)
_arguments "${_arguments_options[@]}" \
'-l[Lists Ruby versions available to install]' \
'--list[Lists Ruby versions available to install]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::version:_values 'version' $(frum install -l)' \
&& ret=0
;;
(uninstall)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':version:_values 'version' $(frum completions --list)' \
&& ret=0
;;
(versions)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
(local)
if [ "$(frum completions --list)" != '' ]; then
    local_args='::version:_values 'version' $(frum completions --list)'
else
    local_args='--version[Prints version information]'
fi
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
"${local_args}" \
&& ret=0
;;
(global)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':version:_values 'version' $(frum completions --list)' \
&& ret=0
;;
(completions)
_arguments "${_arguments_options[@]}" \
'-s+[The shell syntax to use]' \
'--shell=[The shell syntax to use]' \
'-l[Lists installed Ruby versions]' \
'--list[Lists installed Ruby versions]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
(clean)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
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

(( $+functions[_frum_commands] )) ||
_frum_commands() {
    local commands; commands=(
        "init:Sets environment variables for initializing frum" \
"install:Installs a specific Ruby version" \
"uninstall:Uninstall a specific Ruby version" \
"versions:Lists installed Ruby versions" \
"local:Sets the current Ruby version" \
"global:Sets the global Ruby version" \
"completions:Print shell completions to stdout" \
"clean:Remove all downloads" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'frum commands' commands "$@"
}
(( $+functions[_frum__clean_commands] )) ||
_frum__clean_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'frum clean commands' commands "$@"
}
(( $+functions[_frum__completions_commands] )) ||
_frum__completions_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'frum completions commands' commands "$@"
}
(( $+functions[_frum__global_commands] )) ||
_frum__global_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'frum global commands' commands "$@"
}
(( $+functions[_frum__help_commands] )) ||
_frum__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'frum help commands' commands "$@"
}
(( $+functions[_frum__init_commands] )) ||
_frum__init_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'frum init commands' commands "$@"
}
(( $+functions[_frum__install_commands] )) ||
_frum__install_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'frum install commands' commands "$@"
}
(( $+functions[_frum__local_commands] )) ||
_frum__local_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'frum local commands' commands "$@"
}
(( $+functions[_frum__uninstall_commands] )) ||
_frum__uninstall_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'frum uninstall commands' commands "$@"
}
(( $+functions[_frum__versions_commands] )) ||
_frum__versions_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'frum versions commands' commands "$@"
}

