# Physics References

This document specifies the published academic and government-research sources for every physics model in OMNI-SENSE. All models are drawn from openly available literature. No proprietary or export-controlled parameters are embedded in any OMNI-SENSE crate.

---

## Time-of-Flight

**Speed of light in air:** Approximately 299,702,547 m/s at standard conditions (refractive index n ≈ 1.000293 for dry air at 20°C, 1 atm). The exact value varies with temperature, pressure, and humidity per published atmospheric-optics texts.

**Speed of sound in air:** Approximately 343 m/s at 20°C. Temperature dependence: c = 331.3 × √(1 + T_C/273.15) m/s per standard acoustics references.

**Implementation:** `omni-sense-physics::time_of_flight`. Tests verify: round-trip light distance error < 1 mm at 10 m; sound distance error < 0.1 m at 10 m; both verified against analytical solutions.

---

## Gaussian Beam Propagation

**Beam radius:** w(z) = w₀ × √(1 + (z/z_R)²) where z_R = π × w₀² / λ is the Rayleigh range. For practical engineering distances where z >> z_R, simplifies to w(z) ≈ w₀ + z × θ where θ = M² × λ / (π × w₀).

**M² beam quality factor:** 1.0 for ideal Gaussian beam; typical high-quality fiber lasers 1.1–1.5; typical LiDAR VCSELs 1.2–2.0. Published values available in product datasheets and optics texts.

**Power density:** I(z) = P_total / (π × w(z)²).

**Double-pass for active sensors:** I_return = I(z)² / P_total for the round-trip — signal is attenuated by the atmosphere twice. Implementation is `omni-sense-physics::beam_power_density_at_range` with `double_pass: bool` parameter.

**Source:** Saleh and Teich, *Fundamentals of Photonics*, Wiley; Siegman, *Lasers*, University Science Books.

---

## Atmospheric Attenuation (Beer-Lambert Law)

**Transmission:** T(z) = exp(−α × z) where α is the extinction coefficient in units of m⁻¹.

**Wavelength dependence (Rayleigh scattering):** α_Rayleigh ∝ λ⁻⁴ for particle sizes much smaller than wavelength. Dominant for molecular atmosphere at visible wavelengths; negligible at near-IR and beyond.

**Mie scattering:** For aerosols (fog, dust, smoke) where particle size is comparable to wavelength. Wavelength dependence is weaker. Published Mie extinction coefficients for common aerosol types (water fog, dust, smoke) are used for the `AtmosphericProfile` preset values.

**Published extinction coefficient values (approximate, for reference):** Clear air at 1.55 μm: α ≈ 10⁻⁴ m⁻¹. Light fog (visibility 1 km): α ≈ 5 × 10⁻³ m⁻¹. Heavy fog (visibility 200 m): α ≈ 2 × 10⁻² m⁻¹. These values are derived from published visibility–extinction relationships in atmospheric-optics literature.

**Source:** Bohren and Huffman, *Absorption and Scattering of Light by Small Particles*, Wiley; van de Hulst, *Light Scattering by Small Particles*, Dover.

---

## Thermal Blooming

**Mechanism:** High-intensity beams heat the atmospheric column via linear absorption. Heated air has lower refractive index, creating a defocusing-lens effect. Effect strength scales with absorbed power density × path length × dwell time, inversely with wind speed (crosswind carries heated air out of the beam path).

**Model:** Effective beam radius w_eff(z) = w_nominal(z) × (1 + B_N)^(1/2) where B_N is the thermal distortion number, a dimensionless quantity combining absorption coefficient, beam power, path length, wind speed, and dwell time. Published expressions for B_N are at academic precision; controlled operational parameters are not included.

**Source:** Smith, D.C. (1977). High-power laser propagation. *Applied Optics*, 16(7), 1843–1848. Gebhardt, F.G. (1976). High power laser propagation. *Applied Optics*, 15(6), 1479–1493.

---

## Atmospheric Turbulence

**Fried parameter:** r₀ = (0.423 × k² × ∫C_n²(z) dz)^(-3/5) × (cos(ζ))^(3/5) where k = 2π/λ, C_n² is the refractive-index structure constant, and ζ is the zenith angle. r₀ characterizes the aperture diameter at which turbulence becomes limiting.

**Wavelength scaling:** r₀ ∝ λ^(6/5). Longer wavelengths are less affected by turbulence — a 10 μm beam is substantially more robust than a 1 μm beam under the same turbulence conditions.

**Beam spread from turbulence:** θ_turb = λ / r₀ (coherence angle). Combined with diffraction: θ_total = √(θ_diffraction² + θ_turb²).

**C_n² profiles:** Hufnagel-Valley model (HV 5/7) is the commonly used daytime boundary-layer model. Night-time and site-specific profiles differ substantially. The implementation provides the H-V model as a default and supports custom altitude-dependent profiles.

**Source:** Andrews, L.C. and Phillips, R.L. (2005). *Laser Beam Propagation through Random Media*, 2nd ed., SPIE Press.

---

## FMCW Radar Range-Doppler Processing

**Chirp processing:** For a linear frequency-modulated continuous wave (FMCW) chirp with bandwidth B and chirp duration T_c, the range resolution is Δr = c/(2B) and the velocity resolution is Δv = λ/(2T_c). A 2D FFT of the received IF signal produces the range-Doppler map.

**Doppler velocity:** v_radial = (f_Doppler × λ) / 2 where f_Doppler is the observed frequency shift.

**Unambiguous velocity range:** ±λ/(4×T_chirp) for a standard FMCW waveform. Exceeding this causes velocity ambiguity (aliasing in the Doppler dimension).

**CFAR detection:** Cell-Averaging CFAR sets a threshold at T = α × (1/N_train) × Σ(x_train) where α is chosen to achieve the desired false-alarm rate. Order-Statistic CFAR uses the k-th ranked training cell instead, which is more robust in clutter edges. Published threshold formulas for both variants are in the literature.

**Source:** Richards, M.A., Scheer, J.A., and Holm, W.A. (eds.), *Principles of Modern Radar*, SciTech Publishing. Mahafza, B.R., *Radar Systems Analysis and Design Using MATLAB*, Chapman & Hall.

---

## IMU Integration and Filtering

**Madgwick filter:** A gradient-descent-based complementary filter combining gyroscope integration with accelerometer (and optionally magnetometer) correction. Beta parameter controls the gyroscope bias convergence rate. Beta = 0 disables correction; higher beta increases correction aggressiveness at the cost of susceptibility to acceleration artifacts.

Published: Madgwick, S.O.H., Harrison, A.J.L., and Vaidyanathan, R. (2011). Estimation of IMU and MARG orientation using a gradient descent algorithm. *IEEE International Conference on Rehabilitation Robotics*.

**Mahony filter:** A complementary filter using proportional-integral feedback of the cross product of accelerometer reading and estimated gravity direction. Ki parameter controls integral bias correction. Published: Mahony, R., Hamel, T., and Pflimlin, J.M. (2008). Nonlinear complementary filters on the special orthogonal group. *IEEE Transactions on Automatic Control*, 53(5), 1203–1218.

**Gravity compensation:** a_linear = a_measured − R × g_world where R is the rotation matrix from body to world frame and g_world = (0, 0, −9.81) m/s². This recovers linear acceleration, which integrates to velocity. Dead-reckoning position from this integration accumulates drift; external aiding (radar, LiDAR, GPS) is required for long-term position accuracy.

---

## Acoustic Beamforming

**Delay-and-Sum:** For an array of M microphones with geometry G, the beamformed signal toward direction (θ, φ) is y(t) = Σ_m x_m(t − τ_m) where τ_m = d_m × cos(θ) / c is the delay for microphone m at distance d_m from the reference microphone. This is the classical broadband beamformer.

**MUSIC algorithm:** Decomposes the spatial covariance matrix R = E[x × x†] into signal and noise subspaces via eigendecomposition. The MUSIC pseudospectrum P_MUSIC(θ) = 1 / (a†(θ) × E_N × E_N† × a(θ)) has nulls at true DOA angles and peaks in the MUSIC spectrum. Resolution superior to delay-and-sum but requires knowledge of source count.

**Full-echo profiling:** Beyond DOA estimation, OMNI-SENSE analyzes the temporal structure of the acoustic return: peak arrival times (multi-bounce structure), peak amplitude ratios (surface reflectivity vs. frequency), decay envelope (room mode characterization), and spectral signature (material resonances). The combination of these features provides material discrimination capability that DOA-only acoustic systems lack.

**Source:** Van Trees, H.L. (2002). *Optimum Array Processing: Part IV of Detection, Estimation, and Modulation Theory*, Wiley. Johnson, D.H. and Dudgeon, D.E. (1993). *Array Signal Processing*, Prentice Hall.

---

## Covariance Intersection

**Purpose:** Combining estimates from multiple sensors or multiple nodes where the cross-correlations are unknown or partially known. Standard Kalman fusion requires known cross-correlations; using zero cross-correlations when the true value is nonzero produces optimistic (overconfident) estimates. Covariance Intersection produces consistent (conservative) estimates without requiring cross-correlation knowledge.

**Formula:** Given estimates (x₁, P₁) and (x₂, P₂), the CI fused estimate is P_fused⁻¹ = ω × P₁⁻¹ + (1−ω) × P₂⁻¹ and x_fused = P_fused × (ω × P₁⁻¹ × x₁ + (1−ω) × P₂⁻¹ × x₂) where ω ∈ [0,1] is chosen to minimize a consistency criterion (typically det(P_fused) or tr(P_fused)).

**Published:** Julier, S.J. and Uhlmann, J.K. (1997). A non-divergent estimation algorithm in the presence of unknown correlations. *Proceedings of the American Control Conference*.
