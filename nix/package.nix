{
  rustPlatform,
  pkg-config,
  systemd,
  installShellFiles,
}:

rustPlatform.buildRustPackage {
  pname = "razer-cli";
  version = "0.2.0";

  nativeBuildInputs = [
    pkg-config
    installShellFiles
  ];

  buildInputs = [
    systemd
  ];

  src = ./..;
  cargoHash = "sha256-lavypF5UVP3NDq5FufJq9jwsK5FUL0/T06uu2EUZelw=";

  postInstall = ''
    installShellCompletion --cmd razer-cli \
      --bash <($out/bin/razer-cli completion bash) \
      --fish <($out/bin/razer-cli completion fish) \
      --zsh <($out/bin/razer-cli completion zsh)
  '';

  meta = {
    description = "Cli for configuring razer devices";
  };
}
