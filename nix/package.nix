{
  rustPlatform,
  pkg-config,
  systemd,
  installShellFiles,
}:

rustPlatform.buildRustPackage {
  pname = "razer-cli";
  version = "0.4.0";

  nativeBuildInputs = [
    pkg-config
    installShellFiles
  ];

  buildInputs = [
    systemd
  ];

  src = ./..;
  cargoHash = "sha256-JHFYgGR/4l769ltWDMpByYXheW589ykcTNicHp7M4cM=";

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
