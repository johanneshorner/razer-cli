{
  rustPlatform,
  pkg-config,
  systemd,
  installShellFiles,
}:

rustPlatform.buildRustPackage {
  pname = "razer-cli";
  version = "0.1.0";

  nativeBuildInputs = [
    pkg-config
    installShellFiles
  ];

  buildInputs = [
    systemd
  ];

  src = ./..;
  cargoHash = "sha256-RKjWAxSH90sAuZCn/0P9O9yReCvqXOLBUYsZoUl/44c=";

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
