and the proposed method achieves better interference imaging process. The overall process is illustrated in Fig. 1, suppression effect. Compared with image inpainting networks, and the algorithm is shown in Algorithm I. this method holds stronger generalization ability in crosssensor experiments. Even if the training data and testing data come from different sensors, the algorithm can still achieve excellent results. What's more, our method only requires the interfered data to perform interference suppression. Therefore, this approach can bypass the issue of incomplete data. The organization of this paper is as follows: Section 2 introduces the signal model and the network. Section 3 presents the experimental results. And section 4 provides a summary of the entire paper.

- II. METHOD

This paper proposes a network based on domain invariant features (DIFNet), and embeds the RFI suppression into SAR

Algorithm I: DIFNet's pipeline

- 1\. Detect RFI in SAR images;
- 2\. If there are SAR echoes: Locate interference echoes; else: Convert interfered images into echoes;
- 3\. Perform STFT pulse-by-pulse;
- 4\. Predict RFI by DIFNet;
- 5\. Subtract RFI;
- 6\. Perform ISTFT pulse-by-pulse;
- 7\. Convert SAR echoes into SAR images.

**Fig. 1.** Flow chart of RFI suppression network based on domain invariant features.

*N*

*s ft* + sin *ft*

## ( t

) =

## ( j 2πβ( 2π

))

# ( C exp

)

wbism

## ∑ n cn n

- A. *RFI signal model*

*n* =1

Common RFI can be categorized into narrowband  *J f t* + 2 *mf t* (4)

## ( β

## )sin (2ππ

)

*Nm* =∞ 2 *mn c n*

interference, chirp broadband interference and sinusoidal = ∑∑ *Cn nm* =1 =−∞++ *J f t* 2 *mf t*

## ( β

## ) cos(2ππ

)

2 *mn c n*

broadband interference [6]. 

*mk* +2

The narrowband interference can be expressed as:

*k* ∞

( −1)

β

*N n*

Where, *J*

*s* exp + *jf* 2πτ (1) ( β ) = . According to

## (τ

) =

## ( jf 2πτ

)

2 *mn* ∑ nbi

# ∑ n =1 An cn k =0

*kmk* \!( + )\!2  Where, *fc* is the carrier frequency, *An* is the amplitude, *fn* is (4), sinusoidally modulated broadband interference is the frequency offset, and *N* is the number of interference composed of a series of narrowband interferences, signals. approximating a specific form of chirp modulated broadband The chirp modulation interference can be expressed as: interference.

*N*

Formulas (1), (2), (4) can be uniformly expressed as: *s* exp *f* + *jk* πτ

2

(2)

## (τ

) =

# ( j 2πτ

)

wbicm

# ∑ n =1 Bn cn N

*s* exp *ft* + *j k t*

2

(5)

## ( t

) =

# ( j 2ππ

)

*RFI*

# ∑ Dn c RFI

Where, *Bn* is the amplitude, and *kn* is the chirp rate.

*n* =1

The sinusoidal modulation interference can be represented Where, *Dn* is the amplitude, *kRFI* is the tuning rate. When *kRFI* as: is small, RFI is a narrowband interference, and when *kRFI* is

*N*

*s* exp *ft* + sin *ft* large, RFI is a broadband interference.

## ( t

) =

# ( j 2πβ( 2π

)) (3)

wbism

# ∑ n =1 Cn cn n

By conducting time-frequency analysis on equation (5), it Where, *Cn* is the amplitude, β is the modulation coefficient *n* can be found that: and *fn* is the modulation frequency. According to the Bessel *fkRFI* = ⋅ *t* (6) formula, sinusoidally modulated broadband interference can From formula (6), it can be seen that the signal holds global be expressed as: characteristics on the time-frequency map, which inspires us to propose a Transformer network. Moreover, from the interference model in Equation (5), it can be seen that the
