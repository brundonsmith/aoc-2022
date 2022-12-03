mod part_1 {
    use super::common::item_priority;
    use std::ops::Add;

    fn solution(input: &str) -> i32 {
        input
            .lines() // for each line
            .map(|line| {
                let midpoint = line.len() / 2;
                let first_half = &line[0..midpoint]; // first half of the line
                let second_half = &line[midpoint..]; // second half of the line

                second_half // in the second half...
                    .chars()
                    .find(|ch| first_half.contains(*ch)) // find the char that also exists in the first half
                    .expect("No duplicate item found") // should be one in every line!
            })
            .map(item_priority) // for each duplicate char, get its priority
            .fold(0, Add::add) // total up the priorities
    }

    const TEST_ANSWER: i32 = 157;
    const FULL_ANSWER: i32 = 8053;

    #[test]
    fn with_test_input() {
        use super::TEST_INPUT;
        assert_eq!(solution(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn with_full_input() {
        use super::FULL_INPUT;
        assert_eq!(solution(FULL_INPUT), FULL_ANSWER);
    }
}

mod part_2 {
    use super::common::item_priority;
    use std::ops::Add;

    fn solution(input: &str) -> i32 {
        groups_of_three(input.lines()) // for each group of three lines
            .map(|(first, second, third)| {
                first // in the first line of the group...
                    .chars()
                    .find(|ch| second.contains(*ch) && third.contains(*ch)) // find the char that exists in both other lines
                    .expect("No badge found for group") // should be one in every group!
            })
            .map(item_priority) // get its priority
            .fold(0, Add::add) // total up the priorities
    }

    /// Given some iterator over items of type T, create an iterator over
    /// each three-item group (T, T, T)
    fn groups_of_three<T: Copy>(
        mut lines: impl Iterator<Item = T>,
    ) -> impl Iterator<Item = (T, T, T)> {
        let mut next_three = lines
            .next()
            .map(|first| (first, lines.next().unwrap(), lines.next().unwrap()));

        std::iter::from_fn(move || {
            let prev_three = next_three;

            next_three = lines
                .next()
                .map(|first| (first, lines.next().unwrap(), lines.next().unwrap()));

            prev_three
        })
    }

    const TEST_ANSWER: i32 = 70;
    const FULL_ANSWER: i32 = 2425;

    #[test]
    fn with_test_input() {
        use super::TEST_INPUT;
        assert_eq!(solution(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn with_full_input() {
        use super::FULL_INPUT;
        assert_eq!(solution(FULL_INPUT), FULL_ANSWER);
    }
}

/// Shared functionality between both parts
mod common {

    /// Get the priority value for a given char
    pub fn item_priority(ch: char) -> i32 {
        let ch_n = ch as u8;

        (if b'a' <= ch_n && ch_n <= b'z' {
            (ch_n - b'a') + 1
        } else if b'A' <= ch_n && ch_n <= b'Z' {
            (ch_n - b'A') + 27
        } else {
            panic!("Received invalid item character '{}'", ch)
        }) as i32
    }
}

const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

const FULL_INPUT: &str = "sfDRhjhHsHhgWPJvPmmQnmPqnW
pTddGVwcpMTTCdnQJqqQqqqVtVms
MdZCZGdcrCNRFZRhFssL
CttWnSnNfSnCHsWrTlTPPpPCTRrLpl
DgqqghjqJBVgDMTPGVlRGwbfLLGP
cgqBBhjqcBdMcWQcQNnNzsfv
lnDWMgTLlTFlHHgDDgngWFnlBWNcBQrdjcrrdQrPBrdjhWhj
JqSVRRVmmRqJJbZGGJqJvbmBNcjPNQNssQPhSSdwPwwwQr
bCRJqGJJmzmJZRCmFNTLTttTzfFfLglf
SPWvWMvCSPcjzjDbcwfjTl
lLNRNLqhhQVQJlRjrjrDwTzzqzzfrb
GRnRVhRJLFnnhtJQNVdLdLgWCmmZlMlgSCSWSgpZtPBM
pTGFrLFTFWFprLDBmLbSbtmBDb
MqjwqJwZlqJjHlqjHHPmSbsffDmsStDnHnQmsm
ZPJjVPZbVMRRPZwMJZVMNJMcGWpWFcWFFNFGrWTzWzFrzG
MffZZtMTnTtSZLdfgSMtCHSbmWsGwbHGSqvmCqWb
lzpQhrhphhlzDDhRPmBvqHGRbBbwbbssCB
JJljpvhFrrjhptnddMJfdtgnMT
drCtpNLCLpTpJSdswQhvDbHZHDLDHQ
WmWgBWRcRzVVWVBgBBnnlfgWHjmvjQhwbbbshQvZDQQjsHZC
fqBzggWPPzBWBzffcfnMJdtFtrpqrGMpdCCTdM
JwJWqNBNNdzzBSzGsqbdNJbVMpptPmZMrVZrrZMtPmPwDp
THgfgffffHRhQRLVMGVQmGtLDGmM
TjGchhlHhGfhRHgRgWJSqzJWWlqNSzsWSJ
dNmPlzdvdspsFWwQmG
bhZSbVJBJnLNTnwWVHMGwQsGMFFw
RnnbbTnSnSSTTnLfRCCPqDPDNDlfCfDDcr
lhhTcnPchPPHCCStwWTHbS
GDRFNqlQJsGJqGJDqVNsqssDQBSZWHQBCZHHwbZbtHtwCbZW
RNJrFJFJDrmqsVjNmDvrvfzfffcvdpMrvlfh
DtLdNGHNfwBJQwgCrncgpSpcnlfC
sGqWPMPTvPPhTjjsqRqPvSlzFFpjnnSrjczprgllFF
vVPGPGMbPGqTRWsMqZhqvbZNLLmLQddQdmBtBwBNBwNB
ChVzhwpdpqHhtNmHHNHt
QsjGTQcTWQjfjbssQDPmHgfrrVrPZnntZD
jTGJSGvWJwqlvlCBqV
pRVcSRffTPfBWfNVfWBWdJdwhvvwGjjFmGvhLTdh
qsrHqtbDZqsnsZqCQDtHnQQLwFvJFhGJwddvwLcCwJJJdv
sgcqHnzqqzgnHnqrstZzqsnSPllRlVVSNpWVNVRgMBlVPW
WRQTtHrTrrDRvQDHrbtJlpdhLdGsDllfspLpphhs
GzCSqCSmSmVSpsljphlpsL
gVwCVGzmNmCNRQTvJQJHnvwQ
psBDsswNjBcqtHtsTHsqtM
vQrPqZPmvgQZrfgmPrfJQlLvnLzVHLnSnnTLTnnHMt
PRCJRPgmqrmZmmqQQDDRwwNwjwDwpFjDDW
VtBgCqbVjPbSbHtPRdrssZMFZlrRsBRw
LzWmhcDqTDvnDWTFMrwRvwFrGZdGlZ
zJLczJDnWDpDNzmDczWLzWzWNCbbttHPCHbSVbqgHSCjffQf
TjTfvJjjvcjTQcDzMDfQTLLbLgVVVhMrWWblghbbLN
dZHFSpqpqpbWrhhlWh
dFwPBHqFSqwZSmZmSlqZjTvvJTmzcJsTTTjfvsfQ
qqqNTlfjzbMGJlHMSZtZzZgRZDgZDzdS
nLCCVVcmgCdZdSlg
cmQscBVpFsppsVlffQGJHjlWHJGq
whwVGGZhVLwhsFFDCTrDccCctrcctL
CzSSvPSzTBStSWND
HllCHvHJPPqjCPvbfdvvbsmhdRRRsmZhsRdMFQMRFQ
gFCfCVfCsLCftsBsDbSHrbJRJJtrmrddrd
hqQpqWhlNlpMlppfdTRhmbmTdbdHJH
NGvvjvpvpfGgGGDCZZBg
rmBtgdddtqmmrqBGbLGJlmctWWvbNzvfpsVVfzzSVSTsWNpz
RPDRjMhDFljvsvzlSs
wDChhnCQwDwmgJclqrgm
WHrrDbWHQPzNrrRVMQJMQGvvsvvjnDLvfsjsvwfGws
dhdhZhcphZZZmtFFTcZSmcZsnfqjLRnngnpwnGfqfvvnRs
FZcZhtmhFCttldFlSSmlSthQJzNVMbRPWWPJzrbbJCNMPH
wMFBpvTppLpwfNfjggmNmGTj
ddSDDbGHnRDQDZRHSZSdRZDQzjjrzNNNfnmNrllhgglhfgWg
bCQqsJqGDZCHbppwvtVMMvJcLL
pSpSVdLDFCvDDvCFvJgwjsJbNtmtJgSjmj
ZcWNNBQfwjsttsbc
WNQMBflQQNGQrFpFVVRDHpCMDR
PfPvqLphWpWLtZSWpWLPjwJbmDwJbbDbmJVjPQ
lQQnRGMllMjswrmwJM
ggRGFFGGdlGGzFFzzFFcNBvSfLQZShZTtdLWWZWSWZSp
lCfgHsVHJDdswNRmsMRQ
vccvvFVrPcvQNRdnmqdR
rctPBrPBTTpPFBLZZcCCgVHJHVjjbLHfCjfS
dfGdsGGrlFFlbWjfgblhJhLDLDDMLNvJNLLBnmLB
tSppwQQHSSVtwStSpZZVqRJmBDzLvwPmzJznBDDmLBBJ
cHvtpRSvptCRbbjrrcfjrWjf
BBdHdjgQdjMMsHJscFnrzLpLgznLFzcF
wvllmNmVvZfvmZWqcPptPztFSWLFGrrFnt
ZbCvqqmNvflfVTbZfNllsjHdjdhhHDTHRBJjjMsc
FNCPtPtgLFJwPwflFwSrLFcMczQZTbMVmzzVZMcNVVVb
DhRDdhpWQDZmzVSQ
vnhBSHppjRBHqpWvrPFtJLJlLvfLPF
nmcSnnWjmfCTcHPHJCvh
zdDdlrrzGFFLPtPhBBhH
NGNGrzrRrzphwwwMmqqfnsfZZNbSjQSN
lgznQGWQLQWlnSzHSQlwnlDhCbZhZhZChPChwDcDphcb
jTRvVVrMvmLCPNcNZhRNcD
jfftvsrVJLsVvJqsmfqjfjlQgzlQWFzHFGGBQgtFgnnH
sllpwssrsCwrTRgCHGCTcnZD
jjzJtSdhdzbJWhdQqLdzqSHmDZBGZmmcGGgBGTDRBQTD
SVjgVhgtbVzJPfFpvvNrNswV
StzdmmnnjSRRdhPPdZZd
VbTbCqFFMbZTFcNQLgRgQbvvRh
pGsqGGGfHGfZVffzwtrHnmJHllznrS
NLWJvtLjtLzBjNSvSMDCHfwHSlDMlSSHfZ
RTPTVmhpnprfcfgZwgRD
PnPGFhGsTphsFpdPnpVdmhwFvBJzbdWNtJJjtNJNjbvJtttN
RvmgjDqqjqRgZRMRDpQjQhWsbPLPFnPFFbVVLbdSbnPSvP
NwczHBrJTzcBJHrfWJBCJcrCdnPPPNSlLnsnbFnnLlbSFddS
rGJwCCCJHwBGGctGDtphQQMppQWZmRpD
RPhhSMqRccBDZPPPRhPcNZSzzTLJrWZLmVVQLWZdTQQJWL
nwggfwCvbjwvbwpzWLpWVLdrrrQVTm
vgnGGCsCtntbFsgqlRVMSqNVBDtPSc
mtstjJmvTNBcjRRCHCfH
gLpglwwlgHbZbgpgFrdBBBfdfSPBLSSrcS
GQGglGWWgMglQFHgbmTmNtDqnDDVJMDMNJ
ZMbBZfvVfFfBbMvfMhgbfDsrSTTszcldmTTPmcPFDz
QqQQnwrqWQpwRWWpWwJRwNzTTSPpzPPdTPpSPmdSscmS
GjjtJRWtwGQjRZVChZMjVCrMMf
fJNPTvDPTpHHTPwvjNNHDfTWthhgQQGdBddtlvMsMQMvQh
rFbZVZrLmLdGrrhMBQWg
FmnzVRFLqVqqVLVRRFZSFmTwfHHjHCNCCDGwjnCDDfNH
gQHHQJgCnNJnQFQPRbDQzLRR
mwrdpctWtrMvvrrWwGMmGWWPLzFFLSbLnDFsLPdDFbZRLz
vGcmGwBBMGtmmrvlMrGlqNghlVjCCnTHHCHgCCjh
LmLvVjVjsrmrtmmr
tfcnbScRnlMZtHQPCgSQssPdHC
RGGGGnRfcwnGbbJRBRcwJfnGtBDhhVptNhDvLLhVvvjBWNvT
dZWNQZgQbbNvdWGgZvbTfLrjtrPlGJfrLqLJlj
TMmDpwzmVMHpBLfrcccMfqLjct
SwnSBTDDTwwzwnnsFSZdRNbQZWRvgSCvbQ
WPgZgQLLbMgdBrdnGqqfdhVVvR
HzssNTzwlwHHcczwFjMFHjVGrqRqnVThVqrGrGTRqvrf
zFzcHFNlzJBLgMQLJCZL
nPLNcWtNtlLMccLlWdTjzzbBfBQSzqzBqPqS
RbbDZZrGRJhJjgJQSjCfqQCC
rrmRbDDwvZDpprbGrbDvtctlVVVHvdtlcMtWHMHV
DWrZJrQjWwFcrhzVzbpmpcVqhb
MFnFHMNSqbMpMMmG
FNngNRBRCgnHCCHRPvLNdgJWwJDlJJDssZDLWWlWQlsl
BQqNsGrbBCNbNCrMpGpbHhthRCDRDRJCmDVRhRJP
nfvWvcnSWncSTdzzFLJtRmhHmPPVPVTwwHHtTh
WfLfnfSJZJvdLFZWngfBMGMppGMrNBbGMpZjrj
rccMjBMVJcjjjNNqmmCf
LLspTTGsTGntsntTFwnNNfFqQmmNgNqfNQmZvQ
tpDTwlGDTGPPsbtsLsnnqGTJzJrJBzHzMVrMRzBMlShVBR
PsrNPRjjPbjzjLRWLbjmvtCnMntnpfmtNZNCNv
dDlfwwJllhJTcllScSCQvmtCnmtCmQmQmG
TTFcdhJwhBFfwJJHhdchVclrsbWsbzqLgbzrrjgVRgsqgW
vvcvvDJFcDZPTzwfcwSLczzScz
VNnnVVsqGNntsqtBRblqBndSfzCCRzwRfCHSjdfSjzSH
pppsMVlGGhhrZwMMDP
LltNHMZNHMfNnfgtLHWWbhWjcblSbVbcTWVP
vFmCZsqRRBqrVPWsWTWPWb
mQBqJRdqQBqQzzRQztgLgntGZttddLMggw
ZTCCrCWfGLGBWSwHvHHmHvmTTH
bllhnsbjDlqFfqjhnFRppwmvJppmpRRwMNSmmw
FlnFDjdtqhDdfZZBrtBrrPLt
CRCTHHJcCmJgTSTRcSMcRMVstssSrtprppVFtdrdspNb
jjllnvgBLqdsGprtqtFG
vQjzWnWZWjBLhjgwcccRJgZPCmJm
VRNmBBRNRFcCRcFVRSVSqZLLvvlLqvLfzfMhjJLC
TdHsHbDsbHMJLqlLzl
bgQGsgWWGGgbDgwGzBNSFrFtVSmwRRNFtn
pCCggQPPzWnvlDcWVHGJcNBl
LhsLMrwwGlnMBlNG
mmhwZmqSLwjLttnFbvgFTpPtPtgFCz
TtZSJzFZhZzTFcgFFcmRRmJJQllCHvPshVQsCrshsCssHVHW
GjGGDGqdGfbpDBjMdjpBjBNbVHtsWWPHlMlrrrrWWlVlVsCs
dBdDdfqLdBjjFRFScRStmLnL
GtVppGGPbVgTVFQrZzfrJfJJtMJr
DslmNmLsnmNHNNnnqQRZSJSQfqrJzSJn
BNljDHsHlvhmBshDljWsDWlHdgvpVTFggVgGcTTpvFPTzGCV
GRcnTRtcQTcBTsNtpvhFCmmFhZvFPC
bBJMgqWfdwBJfMPPPmvPqhmjvvPC
SMJMdJbdfwJgVglMWWVdcQnBzSQDzGGQzRQDTQSB
mvjVzLgTzVzvVjJrJgrlMhZRFTtRlRhMRRRtFZ
HGqnNNqfnHNGGfCHndBqnqfFlcppsJMZplMFpMtlscRlpC
qSnPGqqbnSdVrvQrrSJjSV
lWFSWZZvVqnqfnSrJzMcPDjJBJcBMPFJ
NGppNgHdHbRsHPbsgGspTwHTMcmMDdJMMmzBDcMBMDQmjMBd
TGGLRGwHsGtpHgHpNbpttwrvCnvCrqSSLvWqPqSSnWvn
jwcqBNNdZLjSfvPdddRlfb
CDVmsgMHCnnDnhVghmDnDCzRRrSrbrlTbsSTlzzlvzPb
gCCFmCWDnChGCFHnGCLBcwwjvZQZNtGqqNZc
LBDcNstdNJscccVDhLHNDHVtFvdldlFvCSnSvjSSbblgvZjF
rWznQqGMMrmmRZbbwvSFgjwbwm
RQnTQfWqqTzTLJJLVtBTsc
SvwCTHqCqqqHtwtnnHHDtWgrBQLzzVLLzSQVFhbrSFLL
cZmPNmPJdmPjPdcclRPPdhBCFVVVrQzQCCLbcgVbBV
fNlmfZfpfWMCtGGpnM
bSNssNssbPHVccPhclPGpP
ffQfZdZZBDDZgLvhmhzVmVppmlpGgh
jdQQQJRljSFFTWCT
lvlLtvnhnfvMgtrvWjmTmPPzjHcrmdcjdd
qCbssCJbppQZQbRJDQSZCJRpzhmcQjdcTBmmGzmdcmjGmdmT
SqwSbJZSpwwFJFDDbqtNVMwVMMlVNgNVlLhV
DqGFQGNMGMQwCcgtCJcr
sVfjWlzzVsmzVZsdVlHrhjppcgpjrhpphcSJ
LRdLsZBWWmlZldZRmzPDvDTTDMGTPFPvBTTc
jzzzpjgBzTDQQHPH
gLLtZVdCdsLfnbZCbdZtHDfHTJJPPmJJfmHQDJqD
bVtWndLtcZgnhsvMSBrMFrvBWNrB
sfqhLDcqfqRRqQhQRqMcvlJpJwFgzwpjplwbgpwzLz
CrGttnhTWtmSnGrtTtSCZGFzbgHHFFFjljHjZHHFwgwl
mBnrrTmWWCGStVCmMcDPcPBqcsRhcvPR
GLZLBNrGZdGGVgMVJVhnvn
dmWlcqcQMWCJVhMn
cdpPqtQbcHlmQjmZswFfTRFpBTfwwT
ZhtZpvbnbpPbtLHLvdsNdcRLNd
jDDjlCflGwsHfdrfTLrrdN
MzmljBMBWPtsbtSQtW
GHrzPSrNLFnMtSBZjZBB
WWbfDmVmwmmlbVDldWslNnBMJJNZZJCtJJJn
vwDfffVvmDwdTvDRQvpLNpLpcRFpphhHLPHg
scsTslgcnCTCScSTcqLLWlFWLLqbGvRbpL
NZMBdBPtNbbrLGqqqGvqZF
NttdbhMPfjQfNtbMbMmNjhNcCzczSSCSJTSzTCScnfnzwC
pjdjCGGGWPCMSDfS
JhFMFcrgBHPnSnWFWDDn
HVBBJctBccghsJhgrbwLGTTdtjLdbmTMMb
DtGHgDPfGfPhfLwNWSSJQcpHcr
dvlMCzdnMRFCCTjnZNpNQJcSbrWzrpSQWS
TVvFJJMjJdlMvRvMClllZZgPtPGsftfDqtGfVGsGtqqq
jSmmcjmJqcBgwmWMCLLzCsMz
TnTQVDGQTpZGNQHDZDHHQDwsCCdLrflsrCVzVrwWzwrr
zDFpppnNQtnTQQvZZZNvnhqqjtRccRbgqqbSSSjPRg
FwClNSwCFstWZLDLvhvjvtjhhD
TmsHmsmrggzmqnnGGvPGjTbbRGBhbB
cHVqgcrVzrQqzHmMcrMnczzcWFVCCFNJZWJZswwFCZWwffwS
mzbsmbmLRCZTRbSJFvPLPJPJpJffcP
QqWqNVNNNllnnWTglqTVlGNPJDvwcJpwfwccPgccPJDfJF
HMGnNMltqGMjHGqMzmTSmzTsRSszSm
qlGDfljllCTgqCTvCDfBHHQsbrSZZHSHWtvWZB
NzpnNpRnLLwRpdwpVhtqQbSbsWQWbSWnrrnH
cFqwFNpLdVcDJlDgccTD
BRqjnSBNBpRHHpjpBSnHnRBQfQzzCvzWrsWCTvfsvCsCCsfC
ZMVbhqbMdlbLTdsWvfPdPC
hlZVDMZcwJNSgjJgJFnq
CZwZssQQZrmsCmNNDpDGFblclD
HMjWMbBVfnnbMbnzMpFhlNSNFFSDcDGSzN
LnLLqjnBMjMngHbnWrTgZsCsgZvvvQrvQs
RCFCCJQbCQcprRlHHPpHhd
tWWLwvswfvZshgqDpdpBgfdf
mZtvZtMpjZzwWFjJTcQQbjjnSQ
fBfVwtttLDFctDtwFPWfTppWfmHCHdJhdChT
bGMRsbsvMQSSzMzZSNzsZvRNWTZJlmgZTJJdhhmppHTJCgTg
jssjNSSGMsQHbsRvHNPjtDcDcLPPPPDwLDDV
pClhQjJccrpbpqHhMhVhSMqHPt
dBZGZdgBzRsBsvMwGGVPVqMGwtVH
ZvDddZvDBdDdDmgCmVmbbCNpCCbljW
DTMCpdCnwRDwdfMCDDCssfZmGrBrjpttjrNrgctmGpGr
VVqJQgSSWzhPGGrPtNNQtm
bFvhgWzHJlDdffswTvRd
jwCCPPTtCswCCNTsqRNbMqQMVvVzMMMQSGvQqn
hprHlmFcHcdhWWLchZzHrLMvvnBvJJSBJMVMnnmMnMMJ
WppLcZdHWHplZWlDHhHTfzRzCCsTTtNNgtjgDw
vhmDFcDZmczMrwcqrMrmDFrvggtVSWgtSNwsjBtNVSnBsjsS
dbbRJHbpCWBBpZVgSS
LZLdHlClPmqLGDvMDv
mFbWsvsJVtbbRwfTSP
BGpQllhLGqhplBGZBfLMTSTLwwfwMJwMPT
GlDnDpQZlZZpZBlpWDNcmrgrWmNdNJvc
zbtqTtHQbZZpqbPpvGJdvQdhrhQjdQGs
qDFLLSNqcWwsGhGDJh
LgBcfnFCSFnLccggSVCVtHZlpqPPtTRMftHMbMzb
hzrrWnzRZRnbWVRzjcRHMDdqqQdNMHqHQQjlHM
sGCpCtppBfCTgwBBCwPBCssQqMQvNlSMMQDQNqHGHvDSbQ
tpLFPgfbCsfbzzcnJhRhZLhc
qzzGqfpFvWFmRSPjPjRP
cwwVssBMtNMNLngstgVBnrsPmHSJJmjllhQdQldmhdrjQJ
nDVSsLwcVcMnBGzTDDCvpvfzGT
bcTbbcZGZLPgTMWZpLLDQnrvPVnVmmjmRPFVrF
HJCJqlzBdsSjzCJRmlrlrnVQQDFnVF
BfwffNdNswLtbWbNcpjt
smJwSNNFMzFNDrvbrbfJHvbl
BRQjqZQcBhrbTsbTnfcn
ZLQRZRBjjPWSsmCdSWMgSN
NhwlDpbWggdSBvBggLFg
fRrZsVfjqljmsQQVmmsnFMFSBLLRvFTFMBSvFF
QfqVVzcsQmcQqrcsNwzzzPphHlwNppPH
nnFdsjVdmpBsBVFHzjpvlTfQdPcQQPGPcvlGPv
DWMDCCWbNJhLtMgJMNLgtMgQflZQlfQGjZZhQZGhTfQcQP
rCrtJJgLLMbgDgMDWNRrWRnzsjpFzBzSHmmqHmqnHH
rmjjJmmdwSmGhdsjJtsgGNzFWQFnBFVWHdFQcLLcNz
RCCbfRlvvPfvCTnHLLnNbNLczHnQ
lqZTllRRpDMlpfZRvgQpSmwwtggQjJgtpS
LDsGvTSSsswCwTrLZDqQWHMWbphlHMpGGpQz
RRPfPRccBdVjPcFlpMpMQWzMWfpF
RjPRjRtczcNBJRSCtLDTvTSDCCST
pqQNgNnSntwgqzzQCzNwCNBRcWtBjZcZGrBMcHMGvWcr
mmJdJPFVbJbPPGZbMRbvvrjcMj
lTMVVlLPfLNQhpgqLSLn
HlBHFrgBvlfzFzqvnvFqpCJbJfQpQpLcmhbcmtmm
jDjPGsRRTMMPjdJmjmLpCLth
MRMZMWsNpFFFVFHW
RGgwWcppGSWcWSRWmGdWcttHQFJHfbQwBQJTJQBQfJ
njjZZCMlCZjqMBFbJQZHJHBQft
DsjCPDDvjFNsMNjNqpGspcsGSmcpccrGWS
cVwMZGVZwHNPgPwRZwHttThlHllvlzQpptzppl
DsCWdqLdDCnfJLSCqsqWRsBdlhjlhzlttzQhhtvlhnhhhbzT
JCWWRWCrLDDdBdLsSsLLSCrCNZMVcmMZMFwMZwNZPZVGFPmr
hhPzDzPhPNbfpzhBbdNbDhttzqWtwttHWwntjqmwmWFm
LgGZSdMMrgTLrZLdgLSgsGTFFjrWtFFmmmFtWjqHFnFtjn
vZgdLvZLZQLRQZQQdMZLdQvVpRhNNPfJDbcBbbhVNJNNhf";
