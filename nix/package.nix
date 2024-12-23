{
  rustPlatform,
  pkg-config,
  systemd,
  installShellFiles,
}:

rustPlatform.buildRustPackage {
  pname = "razer-cli";
  version = "0.3.0";

  nativeBuildInputs = [
    pkg-config
    installShellFiles
  ];

  buildInputs = [
    systemd
  ];

  src = ./..;
  cargoHash = "sha256-fN4HKCZ4d05v2OJx1WqD7r7mkB91c9wQnojHGx2HQK8=";

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
