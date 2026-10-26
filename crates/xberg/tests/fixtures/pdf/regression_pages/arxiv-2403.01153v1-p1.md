# Transfer Learning-Enhanced Instantaneous

# Multi-Person Indoor Localization by CSI

1st Zhiyuan He, 2nd Ke Deng, 3rd Jiangchao Gong, 4th Yi Zhou, 5th Desheng Wang Wuhan National Laboratory for Optoelectronics Huazhong University of Science and Technology Wuhan, China 1

st zedyuanhe@hust.edu.cn, 2nd sheepsui@hust.edu.cn, 3rd m202272478@hust.edu.cn 4th u202014002@hust.edu.cn, 5th dswang@hust.edu.cn

**Abstract—Passive indoor localization, integral to smart build-**

To surmount these challenges, recent scholarly efforts have

**ings, emergency response, and indoor navigation, has tradition-**

pivoted towards WiFi fingerprint-based positioning methods

**ally been limited by a focus on single-target localization and**

[9]. In earlier phases of research, the emphasis was on utilizing

**reliance on multi-packet CSI. We introduce a novel Multi-target**

Received Signal Strength (RSS) for fingerprint characteri-

**loss, notably enhancing multi-person localization. Utilizing this**

zation. However, in Non-Line-of-Sight (NLOS) conditions,

**loss function, our instantaneous CSI-ResNet achieves an impressive**

99.21% **accuracy at** 0.6m **precision with single-timestamp** RSS demonstrates increased stochasticity due to complex

**CSI. A preprocessing algorithm is implemented to counteract**

interactions involving reflections, diffractions, and refractions

**WiFi-induced variability, thereby augmenting robustness. Fur-**

[8]. In contrast, phase measurements exhibit periodic varia-

**thermore, we incorporate Nuclear Norm-Based Transfer Pre-**

tions correlated with propagation distances, thus providing a

**Training, ensuring adaptability in diverse environments, which**

more robust metric. Additionally, the phase information within

**provides a new paradigm for indoor multi-person localization. Additionally, we have developed an extensive dataset, surpassing**

wireless links encapsulates pertinent data in channel responses,

**existing ones in scope and diversity, to underscore the efficacy**

making the combined use of phase and amplitude information

**of our method and facilitate future fingerprint-based localization**

a richer and more reliable approach for localization purposes

**research.**

[10].

**Index Terms—Wireless sensing, Channel state information(CSI), WIFI localization, Fingerprinting, Signal Processing**

In this research, we focus on developing an instantaneous, passive indoor localization methodology that integrates both amplitude and phase information. Employing 802.11ac de-

I. INTRODUCTION

vices, which provide rich carrier and antenna dimensionality, Amidst the technological evolution from 5G towards 6G, we collect Channel State Information (CSI). A meticulous passive indoor localization technologies independent of specialized portable equipment have attracted substantial attention within the academic and industrial spheres. These technologies underpin a wide array of location-based services and applications, extending to elderly monitoring [1], intrusion detection [2], and real-time human tracking [3]. Despite certain advancements made by vision-based methods [4] in this domain, they confront dual challenges posed by lighting conditions and privacy concerns. Similarly, prevalent infrared positioning suffers from accuracy issues influenced by environmental

precision. temperatures [13]. In contrast, traditional radar-based systems are often prohibitive in cost for widespread deployment [5]. In this context, WiFi-based positioning systems, leveraging their extensive availability and cost-effectiveness, have become a preferred solution to address these challenges [6]. Although some scholars concentrate on utilizing Angle of Arrival (AoA) and Time of Flight (ToF) for localization, the resolution of these techniques is often constrained due to the limited bandwidth and antenna count of commercial WiFi devices [5], [7], thereby narrowing their practical applicability.

This work was supported by the National Natural Science Foundation of China (Grant No. 62071192) and the National Key Research and Development Program of China (Grant No. 2020YFB1806904).

analysis of the amplitude-phase characteristics of CSI is conducted for efficient data preprocessing, followed by constructing an effective network for localization, which is capable of accurately locating multi-people. The key contributions of this study are outlined as follows:

- We reconceptualize the localization task as a multi-label prediction problem, introducing an advanced CSI preprocessing strategy to mitigate errors from WiFi restarts, thereby elevating
- we introduce a CSI-ResNet for localization, leveraging single-timestamp CSI inputs for enhanced accuracy and efficiency in real-time deployment. The network preserves CSI integrity, achieving 99.21% accuracy with a 0.6m precision. A nuclear norm-based transfer learning approach is implemented, ensuring adaptability across varied scenarios with minimal initial data.
- A robust dataset was curated, comprising four experimental scenarios and multi-target data from 60 unique locations. This dataset, excelling in scale and label diversity, significantly advances the groundwork for intricate CSI-based location prediction analyses. This paper is structured as follows: Section II reviews
