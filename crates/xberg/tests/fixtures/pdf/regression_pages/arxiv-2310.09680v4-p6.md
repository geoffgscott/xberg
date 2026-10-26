78818:HE 96243:KNOWS 175984:THEM 20256:BOTH

0 1 2 3 4

Figure 6: Example of post-rescoring lattice given an input utterance.

# 5\. Results and Discussion

We conducted experiments on the four types of lattices mentioned in Section 4.3 and observed that their performance in the pipeline was identical. We thus decided to perform a comprehensive analysis based on Lattice Type 1 as a representative selection for the other lattice types. The below tables represent results for Lattice Type 1 on the test sets 'test-clean' and 'test-other'.

LM Scale WIP 0 WIP 0.5 WIP 1

- (a) (b) (a) (b) (a) (b) 7 6.67 17.35 6.65 17.32 6.67 17.41 10 6.75 17.67 6.76 17.68 6.79 17.72 13 7.08 18.35 7.11 18.39 7.17 18.43

Table 1: WERs of Lattice Type 1 post-rescoring with three levels of word insertion penalties (WIPs) on both test datasets clean

- (a) and other (b). LM Scale WIP 0 WIP 0.5 WIP 1
- (a) (b) (a) (b) (a) (b) 7 7.56 21.08 7.64 21.03 7.67 20.07 10 8.16 21.58 8.21 21.59 8.31 21.61 13 9.08 23.04 9.19 23.12 9.34 23.16 Table 2: WERs of Lattice Type 1 pre-rescoring (raw lattice) with three levels of word insertion penalties (WIPs) on both test datasets clean (a) and other (b). From the results, we see a significant improvement in the Word Error Rate (WER) post rescoring. Considering the LM Scale-WIP combination of 7 and 0.5 respectively, we see a decrease of 14.88% in the WER. This implies that our system performs slightly better than the SoTA of the HMM-(SAT) GMM model which has a WER of 8.01% on the test-clean dataset and also beats the WER of 22.49% on the test-other dataset. In our results and discussion, it is evident that our rescoring architecture has played a pivotal role in achieving improved performance. Our rescorer, based on the Transformer architecture, proved to be a crucial component in improving WER. By leveraging the Transformer's attention mechanism, our model effectively captured long-range dependencies and contextual information within the input data. This enabled more accurate and contextually relevant predictions during the rescoring process, resulting in a notable reduction in WER. The model's ability to consider broader linguistic context played a pivotal role in achieving these enhanced ASR results. While our ASR model shows a borderline better WER compared to models with similar architectures in Table 4, it is almost on par with models with different architectures, which exhibits potential. Its advantageous features contribute to its overall effectiveness. The incorporation of contextual rescoring with

LM Scale = 7 WIP = 0.5 Test Set Change test-clean

| Utterances (\#) | WER (%) | Rescored WER (%) |
| --- | --- | --- |
| 2620 2939 | 7.64 21.03 | 6.65 17.32 |

**-14.88**

test-other

**-21.42**

Table 3: WER on LibriSpeech for Kaldi ASR baseline.

Technique Models with similar architecture WER AmNet [26] 8.60 HMM-SAT-GMM [27] 7.19 Snips [28] 6.40 Models with different architecture WER Deepspeech2 [29] 5.83 CTC + policy learning [30] 5.42 Li-GRU [31] 6.20

Ours 6.65

Table 4: A comparative study between our framework and the current SoTA for different ASR Models.

Kaldi showcases a strategic approach to post-processing, potentially refining recognition results. In essence, the nuanced evaluation of its unique attributes goes beyond a simple WER comparison, offering a comprehensive understanding of its practical advantages. We replicated the experimental procedure on the test-other dataset to assess the performance of our model in a different context. Notably, the Word Error Rate (WER) values exhibited less favorable outcomes in comparison to the results obtained from the current dataset (test-clean). As illustrated in Table 3, we observed a significant 21.42% decrease in WER; however, it is crucial to acknowledge that the initial WER values on the test-other set were inherently higher than those on test-clean. Despite this, our findings, as presented in Table 4, surpass the state-of-the-art results [27] for the HMM-SAT-GMM model, which shares a similar architectural framework with ours. This suggests the robustness and superiority of our model even in scenarios with inherently higher WER values. Our lattice rescoring approach surpasses the inherent advantages of transformer models by focusing on context-specific improvements, employing a refined sequential processing strategy for more effective and contextually informed enhancements. By utilizing a transformer-based model, we aim to capture semantic information more efficiently, particularly in scenarios involving Out-of-Vocabulary (OOV) terms. Furthermore, our approach enhances contextual relevance in transcriptions by prioritizing lattice paths with higher language model probabilities, resulting in superior transcription accuracy. This targeted approach represents a specific contextual improvement, addressing challenges associated with OOV terms and elevating the overall contextual accuracy of the ASR system. Additionally, our proposed approach introduces a novel perspective on lattice rescoring techniques within specific ASR contexts, contributing to a deeper understanding of their broader implications on ASR performance, even in datasets lacking contextual information. Runtime and cost for this model-The experiment, incorporating an H10 GPU in the speech recognition pipeline, incurred
