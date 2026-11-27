# PowerShell completion for grandjury
Register-ArgumentCompleter -Native -CommandName grandjury -ScriptBlock {
  param($wordToComplete, $commandAst, $cursorPosition)
  $subcommands = @('tally', 'stadium', 'cost', 'agreement', 'compare', 'census', 'ledger', 'dissent', 'swing', 'minority', 'bench', 'pool', 'aggregate', 'canvass', 'roll', 'muster', 'margin', 'accord', 'standing', 'answers', 'pairs', 'shares', '-h', '--help')
  $elements = $commandAst.CommandElements
  if ($elements.Count -eq 1 -or ($elements.Count -eq 2 -and $wordToComplete.Length -gt 0)) {
    $subcommands | Where-Object { $_ -like "$wordToComplete*" } | ForEach-Object {
      [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
    }
    return
  }
  if ($elements.Count -ge 2 -and $wordToComplete.StartsWith('-')) {
    @('-h', '--help') | Where-Object { $_ -like "$wordToComplete*" } | ForEach-Object {
      [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterName', $_)
    }
    return
  }
}
