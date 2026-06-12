use vello::wgpu;

// ── Error ─────────────────────────────────────────────────────────────────────
#[derive(Debug, thiserror::Error)]
pub(crate) enum GpuRendererError {
    #[error("aucun adaptateur GPU disponible")]
    NoAdapter,
    #[error("erreur surface : {0}")]
    Surface(#[from] wgpu::CreateSurfaceError),
    #[error("erreur device : {0}")]
    Device(#[from] wgpu::RequestDeviceError),
    #[error("erreur vello : {0}")]
    Vello(#[from] vello::Error),
    #[error("surface texture indisponible")]
    SurfaceTexture,
}
