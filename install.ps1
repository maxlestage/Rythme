# Installs Rythme on Windows from the ready-to-use program of a GitHub
# release, after checking its SHA-256 checksum.
# Installe Rythme sous Windows à partir du programme prêt à l'emploi d'une
# version publiée sur GitHub, après avoir vérifié sa somme SHA-256.
#
#   irm https://raw.githubusercontent.com/maxlestage/Rythme/master/install.ps1 | iex
#   .\install.ps1 -Dir C:\Tools\Rythme -Version v1.0.0 -NoModifyPath
#
# Parameters (or environment variables / ou variables d'environnement):
#   -Dir DIR        RYTHME_INSTALL_DIR      folder of the program
#                                           (%LOCALAPPDATA%\Programs\Rythme\bin)
#   -Version TAG    RYTHME_VERSION          a release like v1.0.0 (latest)
#   -NoModifyPath   RYTHME_NO_MODIFY_PATH   leave the user's PATH unchanged
#                   RYTHME_DOWNLOAD_URL     where releases are downloaded from
#                   RYTHME_LANG             en or fr (otherwise the system's)

param(
    [string]$Dir = $env:RYTHME_INSTALL_DIR,
    [string]$Version = $env:RYTHME_VERSION,
    [switch]$NoModifyPath
)

# Windows PowerShell 5.1 reads a script file without a byte order mark as ANSI,
# and this one is UTF-8 (for its French messages): then it reads it again as
# UTF-8. (`irm | iex` already decodes it as UTF-8.)
if ($PSCommandPath -and 'é'.Length -ne 1) {
    $global:RythmeInstallFromFile = $true
    & ([scriptblock]::Create([System.IO.File]::ReadAllText($PSCommandPath, [System.Text.Encoding]::UTF8))) @PSBoundParameters
    exit 0
}

function Test-RythmeFrench {
    $lang = @($env:RYTHME_LANG, $env:LC_ALL, $env:LC_MESSAGES, $env:LANG) |
        Where-Object { $_ } | Select-Object -First 1
    if (-not $lang) { $lang = (Get-UICulture).Name }
    return $lang -like 'fr*'
}

function Install-Rythme {
    param([string]$Dir, [string]$Version, [bool]$NoModifyPath)

    $ErrorActionPreference = 'Stop'
    $ProgressPreference = 'SilentlyContinue'   # much faster downloads

    function T([string]$en, [string]$fr) { if (Test-RythmeFrench) { $fr } else { $en } }

    $isWin = [System.Environment]::OSVersion.Platform -eq 'Win32NT'
    if ($isWin) {
        switch ($env:PROCESSOR_ARCHITECTURE) {
            'AMD64' { }
            'ARM64' {
                # Windows on Arm runs the x86_64 program through its emulation.
                Write-Host (T 'Windows on Arm: installing the x86_64 program (it runs through emulation).' `
                        'Windows sur Arm : installation du programme x86_64 (il fonctionne par émulation).')
            }
            default {
                throw (T "no ready-to-use program for the processor $($env:PROCESSOR_ARCHITECTURE); install with Rust: cargo install rythme" `
                        "pas de programme prêt à l'emploi pour le processeur $($env:PROCESSOR_ARCHITECTURE) ; installez avec Rust : cargo install rythme")
            }
        }
    }
    $asset = 'rythme-windows-x86_64.zip'

    if (-not $Dir) {
        $root = if ($env:LOCALAPPDATA) { $env:LOCALAPPDATA } else { Join-Path $HOME '.local' }
        $Dir = Join-Path (Join-Path (Join-Path $root 'Programs') 'Rythme') 'bin'
    }
    if (-not $Version) { $Version = 'latest' }
    $base = if ($env:RYTHME_DOWNLOAD_URL) { $env:RYTHME_DOWNLOAD_URL } else { 'https://github.com/maxlestage/Rythme/releases' }
    $base = $base.TrimEnd('/')
    if ($Version -eq 'latest') {
        $url = "$base/latest/download"
    } else {
        if (-not $Version.StartsWith('v')) { $Version = "v$Version" }
        $url = "$base/download/$Version"
    }

    # Windows PowerShell 5.1 does not always enable TLS 1.2 by itself.
    try {
        [Net.ServicePointManager]::SecurityProtocol = [Net.ServicePointManager]::SecurityProtocol -bor [Net.SecurityProtocolType]::Tls12
    } catch { }

    $tmp = Join-Path ([System.IO.Path]::GetTempPath()) ("rythme-" + [System.Guid]::NewGuid().ToString('N'))
    New-Item -ItemType Directory -Path $tmp | Out-Null
    try {
        Write-Host (T "Downloading $asset ($Version)..." "Téléchargement de $asset ($Version)...")
        $zip = Join-Path $tmp $asset
        $sums = Join-Path $tmp 'SHA256SUMS'
        try {
            Invoke-WebRequest -UseBasicParsing -Uri "$url/$asset" -OutFile $zip
        } catch {
            throw (T "could not download $url/$asset ($($_.Exception.Message))" "impossible de télécharger $url/$asset ($($_.Exception.Message))")
        }
        try {
            Invoke-WebRequest -UseBasicParsing -Uri "$url/SHA256SUMS" -OutFile $sums
        } catch {
            throw (T "could not download the checksums $url/SHA256SUMS" "impossible de télécharger les sommes de contrôle $url/SHA256SUMS")
        }

        $expected = $null
        foreach ($line in Get-Content $sums) {
            $parts = $line.Trim() -split '\s+', 2
            if ($parts.Count -eq 2 -and $parts[1].TrimStart('*') -eq $asset) { $expected = $parts[0]; break }
        }
        if (-not $expected) { throw (T "$asset is not listed in SHA256SUMS" "$asset n'est pas dans SHA256SUMS") }
        $actual = (Get-FileHash -Algorithm SHA256 -Path $zip).Hash.ToLowerInvariant()
        if ($actual -ne $expected.ToLowerInvariant()) {
            throw (T "the SHA-256 checksum of $asset is wrong (expected $expected, got $actual); nothing was installed" `
                    "la somme SHA-256 de $asset est fausse (attendue $expected, obtenue $actual) ; rien n'a été installé")
        }
        Write-Host (T 'Checksum verified (SHA-256).' 'Somme de contrôle vérifiée (SHA-256).')

        $out = Join-Path $tmp 'out'
        Expand-Archive -Path $zip -DestinationPath $out -Force
        $exe = Join-Path $out 'rythme.exe'
        if (-not (Test-Path $exe)) { throw (T "$asset does not contain rythme.exe" "$asset ne contient pas rythme.exe") }

        New-Item -ItemType Directory -Force -Path $Dir | Out-Null
        $target = Join-Path $Dir 'rythme.exe'
        # A running program cannot be replaced on Windows, but it can be renamed.
        if (Test-Path $target) {
            $old = "$target.old"
            Remove-Item -Force $old -ErrorAction SilentlyContinue
            Move-Item -Force $target $old
        }
        Copy-Item $exe $target
        Remove-Item -Force "$target.old" -ErrorAction SilentlyContinue
        if (-not $isWin) { & chmod 755 $target }

        $installed = 'rythme'
        try { $installed = (& $target --version) -join ' ' } catch { }
        Write-Host (T "Installed $installed in $target." "$installed installé dans $target.")

        $sep = [System.IO.Path]::PathSeparator
        $envPath = [Environment]::GetEnvironmentVariable('PATH')
        $inPath = ($envPath -split [regex]::Escape($sep)) -contains $Dir
        if ($inPath) {
            Write-Host (T 'Try it: rythme learn' 'Essayez : rythme learn')
        } elseif ($NoModifyPath -or $env:RYTHME_NO_MODIFY_PATH -or -not $isWin) {
            Write-Host (T "$Dir is not in your PATH; add it to run rythme from any folder." `
                    "$Dir n'est pas dans votre PATH ; ajoutez-le pour lancer rythme depuis n'importe quel dossier.")
        } else {
            $userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
            $newPath = if ($userPath) { "$userPath;$Dir" } else { $Dir }
            [Environment]::SetEnvironmentVariable('Path', $newPath, 'User')
            [Environment]::SetEnvironmentVariable('PATH', "$envPath;$Dir")
            Write-Host (T "$Dir was added to your PATH. Open a new terminal, then try: rythme learn" `
                    "$Dir a été ajouté à votre PATH. Ouvrez un nouveau terminal, puis essayez : rythme learn")
        }
    } finally {
        Remove-Item -Recurse -Force $tmp -ErrorAction SilentlyContinue
    }
}

try {
    Install-Rythme -Dir $Dir -Version $Version -NoModifyPath ([bool]$NoModifyPath)
} catch {
    $prefix = if (Test-RythmeFrench) { 'erreur : ' } else { 'error: ' }
    [Console]::Error.WriteLine($prefix + $_.Exception.Message)
    # Run as a file: fail with a code. Run with `irm | iex`: keep the window open.
    if ($PSCommandPath -or $global:RythmeInstallFromFile) { exit 1 }
}
