use std::collections::HashMap;

fn main() {
    let input = get_input();

    // part 1
    let rucksacks = input.split("\n").collect::<Vec<&str>>();

    let mut priority_sum = 0;
    for rucksack in rucksacks {
        let rucksack_char_vec = rucksack.chars().collect::<Vec<char>>();
        let rucksack_sides: Vec<&[char]> = rucksack_char_vec
            .chunks(rucksack_char_vec.len() / 2)
            .collect();

        let left_side = rucksack_sides[0];
        let right_side = rucksack_sides[1];

        let common_char = get_common_items(left_side, right_side).unwrap();
        let priority = get_priority_score(common_char);
        priority_sum += priority;
    }
    println!("Total priority for pt1: {}", priority_sum);

    // part 2
    let split_input = input
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|rucksack| rucksack.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let three_rucksack_strings = split_input
        .chunks(3)
        .map(|rucksack_group| rucksack_group.to_vec())
        .collect::<Vec<Vec<Vec<char>>>>();

    let mut priority_sum = 0;
    for single_group in three_rucksack_strings {
        let common_key = get_common_key_in_rucksacks(single_group);
        priority_sum += get_priority_score(common_key);
    }
    println!("Total priority for pt2: {}", priority_sum);
}

fn get_common_key_in_rucksacks(rucksacks: Vec<Vec<char>>) -> char {
    let set = rucksacks[0].iter().collect::<Vec<&char>>();
    let set_2 = rucksacks[2].iter().collect::<Vec<&char>>();
    let common_chars = rucksacks[1]
        .iter()
        .filter(|char| set.contains(char) && set_2.contains(char))
        .collect::<Vec<&char>>();
    return *common_chars[0];
}

fn get_common_items(left_side: &[char], right_side: &[char]) -> Option<char> {
    for single_left_char in left_side {
        for single_right_char in right_side {
            if single_left_char == single_right_char {
                return Some(*single_left_char);
            }
        }
    }
    None
}

fn get_priority_score(char: char) -> u32 {
    let mut char_vec = ('a'..='z').into_iter().collect::<Vec<char>>();
    char_vec.append(&mut ('A'..='Z').into_iter().collect::<Vec<char>>());

    let char_hashmap: HashMap<char, u32> = char_vec
        .iter()
        .enumerate()
        .map(|(index, single_char)| (single_char.clone(), (index + 1) as u32))
        .collect();

    return char_hashmap[&char];
}

fn get_input() -> String {
    return "QLFdFCdlLcVqdvFLnFLSSShZwptfHHhfZZZpSwfmHp
rTJRjjbJTgzDJjdsRsfwtfNwtfmZpZNhmmzt
bMdJjsjglnVMFCCc
BZvZMBBBMTtZTgcCPdgtgQCrrV
VspNDDpsGDGnRmRpRplQdrrPcPCjgDCcPQCQQj
RVnsmspnpwFRlHGRwHHlnRSThSSvBTbFTZMqTMZMTZFh
ttffrVJWtWpgtQnZGVnNSLTHSZ
jRsjjmhdRcjcRsFcdwGLMZSnHMTSMNZN
RjczlvjhPCcPjcvRpbglWplJBblqrGgq
NwCWwdNQNDCwGpWNQwmJtZgPZrdJgLZcPhLddr
blqpnFTqrLbcLPtV
MnjqSSpqlFRvSqNDGHvWHQDwfWmm
jfLljlQhDLmlrMJQtJTJrQqQ
NpBbjjsdMCgCCMrb
dwspwGnSHHGsGzDDlFDjVWjfZWnP
wQhTZwvpZFZdqWLnnwSrWC
mfDmMFlDcPLdgDSCLCqg
PmzclsMclMlFsHHsJZFHpT
LfLJWNdJnBLfhndfWdZqcgDZgSqgCCSSSLDF
bQVQmrrjPqQDZSZBCQ
RRtGjVmRsPbPrrnNNpNHHnBnpHns
PfbGNwGBwNcPTbGNQFBVjsjztVztVjHV
hrdCJhmlJhZrLDRmghrmDrzqFsbgtbHqnznzznQtbjtz
WdZdDJCDmrJmLZrLDLDZlClcSccwSPbNPfSNWfGNNWMGNc
QwrnTSgqgFShSdfHPdbS
BGdjmMmZMvfhvCZZPf
BzGmzVGGGzmzGpVBtdnQqqdTQQDDqrpR
PPRPwLQlLtbPmbwgJhrSssNlhhrgsZ
fFTdFvTMNfzVnFqdnjgSSjjsSjhghpJs
dvczcFzNTWVWMFLcLbQQwmbHLCLL
HhLLDfMmTjsjmLmhsmmfZMjGtpGJtdcvnCWtZJcWGddttW
gwrwwgzwgDpJddrJDr
SBwBBbgVsVmRRhDM
SZdmfdZRTQZTQgHVVGGRqZdVCjCcNCNcJRWcCBbJjCPCsNsc
FnhzMMhDDFlzlnvpwMLrMDCsbcjNJbcJbBcBfPhCNbWj
wzwnpDDnLvFnLlttLrzGgTVGQqZtTqSqSdfZTg
FJJWWWrCGWdmzFlTVqqlMhmvVlNh
btpgtfjZjjhgggrNMThl
DpwpfRZDZwwfwjsnjfsZnnnwGcCRCHcCCCLGHLWcrcFCWCHW
ljHHHBtjQthchhZpqqNt
FTmJnPFwzlJPmzTgTgbFwJbMCpbMchhhqbhWCDMDhZcppM
JwlFGwVGnFFGBGjdSsfdsG
QsvpGhjGgswvjjwjjjvPpThJfLZCCLCSSLbFLStCFCSgtH
mDrzdzMqqnMrDMmZnqnzNVRCStlCHFSCtJqlCLFCFLfJfJ
mBDzNRWBDDMBpGsZcGZWGjPp
SlLQhQsvvttFlWsWcfHHMTJfwSHwTfTf
VZmmrRDRfpTHJcRf
jzBnDDgjPchWlsQsBW
LTLVdTSLNTLnTSnrWvdwswsfmJwmwGsffH
FbgCbRRppCpPbgMcZvCcGftGGltwHwGtplQQsfJw
CRBMCvZZRgMgBbDCPcDrjLzWLVrSSzShSSNrBS
hVJJjhjRVRZjQvDfBstsBVNBdwstHsld
pCTCcMqCThTFLFFPWcWSPHtwwdmcBHHmNtHdmwmwBl
rMTCCWPLLPCMFhDnDrjzRrfDJD
pqMpCvMchvFNWSBdVNqQ
zDRJJDGJJtNtmGRRWVdFWWVdSfjb
DDJLmnJmzwGmGLTPhTCNpgcrpv
cpPpbPWVprWcbJrrwpCwwdWrvNNFRqzNnChgqFzFnZvqFlzq
fTtHLfSHSsNDGLSmsLvnhFqzhzlzDhhvRlFz
mSMLHTQTmHMSfBSMTPdBJJNNVddrVrbjbJ
zpCpBTnFgFbncznbblzdhRswdlJsLllJdw
QqqmtWVPWvHDVmqDhjsljwRhlZldhRMQ
tWSHDmVfmrtPHVgGRRbgTRpSgpTc
ssTbzFRtPRwTFZtvbPRMhndBqMMvMBHJnnHMMd
WQVWzlGWVBqqdMQJMq
pVSpSSgLfjDzWrLGWWjDzzfLtbRFFNtPZRssspsNRZcRsNZb
jnPzzGlnnznWnzhvGnnpVFrZmVFcgjrrmZRFtj
fsbgTdwdqBbfwCptVtdZRcrRCp
gsMbgfHsBSwsGhhJWMLnWPPJ
bpmbJpNbbbNGGmRJzJTsfdsvsNdglfhssCvC
hWLwQjZjLhjHFFBLZldvvflrvtfjCrfjlT
QWVQZZFDcDJJhJJc
RmRghgRlNgfGGRmdGqhsgsZFZZpBvHpZppHcgH
tbLCDLnLtSbbbjtPtMLtDPTvHHBpcHcsHvTcHsmZcF
rSLrMJzDznzGmhNlVwdrRr
vWjljMWcnSSpjmzbJVzJrTCmtGJV
NZDDQLRqPJrPzrprTC
gqDqqwpdHWhlgnjH
ccptcpstDvbNvHbLNRZZ
dFjhdnjQFJlFCQSjgngJPJgWWrRNWNRtNCzrVbRzNVzZZL
PhThFSPPSpsmTcqMwt
cLcLlMhGMGcpGzslHFHFvnHlBDvWbT
VHdQwqPJdPwjJPdPQRrmjjjQnTFrbvNWFFffDvbvvNDvbBNN
RwRJCHmmdQJZZLzGphcCtz
hVvFVjvjVWmFRQVZqTpqtwQpwpqZfp
gvDlSBDJSlPLcLdDwzwtptqTTTzwcCCt
JgrJGbLgvnWsmvVr
rwmqqRqrnHQGmnjCCqCzdBzCBJBz
hFLgbWWPWmvtLhPtgpcdjJdcBJdpJjDsgp
lvfhSSPWtTNTTZZmfr
bHDDssRHsjNMbJjJLQJsbTtGvSCzCGQCTzGvSqSBzT
mmVrwhmmpfPStnTSBnhStG
pcwrptZcgFcmpgHRDjjZlDJsjbbD
JJRrmFqJMdFFJMjjJcqGgzSCSHSCscPCHPHGZc
VWpWptnvSmpPGCHC
vQnDLBmbntvLBbnlldTQFlJlFFrNRd
LPDftnHFQfwmBcBGmc
CVqRsdqvdrlsCVsNvqwwSpTNSSDSDDBBTTSN
lqlDRddjbblRbRqrlRRjsbvghHHnPQWjHWQWZHPHWZhFnP
bwQsDcgsJqcsDpcQRQnpqtVSVvgSMMMfMvfVBVfdvM
CGZFrHHPrTZNGGZZHmCZHlVfjfzjSfzBtBBNSBVjvntf
ChrCCLGrTlhJnhDncRbp
nmFnhfTQjSzfjddZWsRRRFRFGl
HDgCwgtQbZlHsrqHHr
cJPCgCCPbpbgDMPvMQnjmnhTfmzLpQQmjz
QFHSQdNMCSgcSgFtttPNFJpCpnTjZlbblpppZplZjz
LqLsWMRRfrrWMmMGpbTnbppbZnTjpnmm
fWBrMqWsGswGfGRMMwrLgtPPdNFBQPHNHNPPcFPP
dngbSppJSSppbVMZQQMjqfQQgwcl
TWmSWtvCRCWjwfjqQqMstq
FhFvRzSTNmhHnVPhGhBJdBpB
gcHPgzGmPPwTsSTsbwbdWD
QjBLLfVhhBqqBFQLrLjVFlNpNDtsSWTDdNptdbqbdS
jCMFLVjFBFjJJLFFMVBFrLnvPzHRmHPGnGWWcCvzHRZm
PDPqWWjhPpPbCsjwjTVbLT
SrtCttGRMddSVwHFSs
JtfvttmrGMRRJzJCqhqqqWQZhCNgJZ
ChrCVFQCVQlwQNwpQcmmcjmWBmddghjjdW
sbDTZStTqqfSBggPmWjWWNsL
THqqSHDTtZTDTHZZbHTzRzFvlFCVprFprQVnCzNppQrz
PdfWCwMWjPSrdgCMnnlGsGQvvpJZvFGnps
DmBhVBLbbVqVBzTRLBRzzTLNNpRQNNZQZppZvlQpZvllvF
zVDtVHBbbTbzDbrjgWjMPtMWPMlj
JLsTTNDsgTMNvDQpLpGpLGNJShrfzCFnSnSrnfzCfTFhWrfw
ZcqrRddHZZVRfzWnVWCzWFnn
tZHZtrHHPdRtdHlcccQggsplpJDNvMGNGMss
cMCLfStfMTCjPMPcGzjftMbgsRNmRgmmGsmnJbNJbghJ
QHVVWrFFWZNShHSgbSJm
qZwwrrpqpZpZFvqrQdFlQVSwLBMBfTTLTjLBCcdTMzMftPPB
SwsdBTvgvJLPNptpCpCmBDtn
wffrzwGFWFNZWpjWZnNm
zrfflbRwJPhbPbsS
HjHHRtwjnjRblQRttHwQGvGWNNBWvqGzfTvfNN
FmScCcrsdVZrpBrVcCVFzffvzzmWGLWgqWqgGWzW
SFVSDDBdsdDSJhnjJltJbPtHRM
FjGFVqWrzQFlQrZzGQzFLTvfwwTgMnvcnbRMLRdnfb
CCttSNsSnRfgncSg
CNspmDBPtPmJJNBJPNpDhQZVzQlhqrGZflfVjQFrQj
dNNdHWcmdmPPptmmWHpPTFFjJPGrQsVsPQGGGJVDrVVGrS
MhZlZhlgflgfnfDtjbjJGbtnVtGS
LtZqlzhzqMZWHHLwdHmFWp
llNRlfwWRwwLlwFNNgRrVCBjdjCVdjpWjtVWCD
HTQqzPqzQPmhhmSPznSsssJtdnMZddtMCjprtMjCnBVnjZ
PzHQmqsGSJPSmQqPbfwNcgNbNgNfBGwR
lPdzlZPzQzMZQGQrTZvvpjHTTpfsTTZb
zRShhtWRnqnqSNRnDTTHvfNJspNsLpTsjL
hBVncVtDSnhDnDBBtGrlzwmmMlGmVrMdrP
HPTZVHVPlHDPlfgnjJFdJdjPjSPqCS
hLRRBhwGhqbtmsRSSSjjdMJjnJGSMj
QrQtqrRrcQDgVglc
ZTwbbZdchZZjmVWHTrHWBVJtBB
glslCDqLLDfGRqlsgLssfrCHBHFHmrHBBppFmCJWWp
fRzvvvgGgNSNvmQbSQ
qPGGPwCTqTzHCvPGqWdLFLssLpstLLspvd
njJchhcbjbDrbcLNlLrpWWrLLHgp
DQhMMMJQMQJnVbbnRHSMPwZmGZPZRTRCwTZmZGwz
zzGNfPbcgdPqLrqvWWVzMq
DGmJtnJTJRhhJMhCQqCLCLrrLM
ZnHDtSZlTBHnBdccGSfGcwjjdb
FpZDpQZDvMwZpCCMdCBPpJGPPLgJGGLffJJL
jlbswNrlPPJJfGlf
bnNwqbHnNwRSrqhbdCcmHddQzddFDdvZ
gbQQQngWPVVtvvPQNVNvWWSHGwDsCCmDtHSlmrssDmHs
fqhMLFFMMZqZMRZqMjRMqLJSCdFlrrldsDrCsDSSHHGCSC
MJRZLZLGMcTqczjNPzNnzPvWBVgNnP
gqdbBffTvlRHbwLl
nMMQJQpGdsFpQsJzNMRLLDlmLLmjFFmLjDRF
pzGMnVcMBfTdtBBV
WSbfmrrrrWdbWmdfDSSStmHjtMtvCLVnqBHCVGtVGnMM
lRcgFRZhJgnMLjvGgv
lcvwTcFTplvwphzcTTJTbsdsPSPDdbmzmDSWPsSm
bbdTjTQTQMsZNqqhJrZslg
jFGVjwfCPVGfwjCVqWhWZFgqWrglllNN
PjfSPzRBjCCfSBCGBLznTndHcdMLbMmmdT
wSVMJSVccdGwGnsgbVTTbRsCRNgN
rHjhHLmrhPJrqjNTRDgBbbRRRs
zqmPPqqpPLzltrMdJcZpfdpGWWJJ
ZhrBBJGrgJhGHttGGVPPcPPF
cnzLqNssfRnpfWqsLfcfWQNMbMVPDtnDtbHFtMbPtVPFFM
jfqzCCLsWQLcjgldjmljmgTd
wghGSSGZPVwgqtwtwCCtFFMM
BvbspnBznvvWHWHHHbCQptQFQlFcqMClqLLq
JWzzsJHWzfWjJrvMBWHBBGDmVDrVhZmmgSPSmZVVrh
ccRMJRsjjgJgcPCSCCVCwsSWVNzp
WQQqnmrBWtqWqdSbVwwBSpbbCSBB
QvDqmqqmgWPWjPvW
msqpjDWspRWwvFvDWWhnbbJfPzFQblJJPlnz
gGGrMTgLVBsBBLdsVTrSCBffHQfdHhnbPPPPffndlbzh
ZCVsCGSScsLZpwNpqmZRqW
PPsGmJPVPQPZmsQCVPJPnPCMDcTcdqDDTqvFhvnTjRDTDchq
BdrtzNBLHStHrdrlwfNThvFhcvbDccThjbFBqq
SSgdHNfSHHgzLHtLNWSPQQPMQVpmmppVCmQZCW
pPssrWWLdndHPJdd
QNQFTLNBFTzzgjfGTjffFNZjCSGnHDnSDJHnDScttDCcDnmd
FVzVLZwZZgswqqrbphbR
VpWCZjCwWnppZpqnhNjjNZjFLtLzQJHdHLQRzWLRzRztHJ
DMGPmPMgTSmsgQzRFbdHRLJgdn
csDMPMGDDvMSSPnDTvrDChhwljlqNNjchNCjNVcf
WpGGmbSGpVWWpjMMTNdfCFNdFfRNwNSF
JsQztzrvrJqsTTRbbvFBhhhv
cLrDqLccsLqbDHGpZWDHgjGlZW
QGMQJMmsJmMCmmqjsRvLvvdgvgVvDVdD
BDcrcNbNppwTpzRdvvchhFvfFv
plBBwWrbpQHDjGmGJl
mzFlTdmSDzrPvCJqqDVVNC
hfRmhgjRhnfwnRHcnhGGvPJQPvvfLfQvNLGv
BhhnjMgRWghpwjRWMRjrZzdbSbsdstTrltdmMs
bLLnbqjpvplnDvNlqpqBWJZSdPJCNdJJThhSPhTd
HFwHHQMMFHGzGwRPPJPTWthTZtJSQr
mfWMHFHWHmgmFcwGwwpbDljqjBDcDnLcVnlb
wBrWBwSWRJMBwdZnPQPgFnwGVF
fLjfbsvDDfvvqqGqZGqmPQgqTGGG
vZLsjzjjZCzJWRNSBR
jTRbRHHqPqTRBHqdjhgvgghhZQdDvvgvhC
WLWWzzFszsmNFGWSFmMrpghCtZvhlQNDgQCDgctC
FJsLsSrDmsFSDLWrzJmmMsGqjRBVbJTBVPVBbBqRjPBjHn
QbwwnDDQDcDfSbDbfhhrvrCtJMvJSCvvJh
FWRjjLjmdZWdWNBFNWNlNQQrMGvvMGgssGvQRvrMJs
BjWdlBpmdmBWFWdpWfPfpVnVwfHpqPQDbq
SqrvlMldqvSWdGPTGzWpWpzpHP
tRwmhtbsRRFsLwGGTVDHppTNdbVp
FRCRQdCFtCLmBhCcmmQdhFdCvnfjffjZlZnjSnvfcSrrgMgn
GQQtNJQWWcqPPhMMtwqD
WpWLlBWZCvhjwMMZqDDP
WgvmLVmHCbpppLgdllHddvCmFGzGnfsJJQJsJncSsccFVffF
HcSsSlTTvvPPWWNMWWgPTPPbGbbrwJQbrrDphrHJJRpRhp
ztfLqqzmRwDGlLDb
fdVtmqjdZBmSvjsPSWlTgv
DPvDhhMRRMhRNDLPMNsbwHwrjgnddqddrWdPtHzr
pcBGSpcVBfJWCcmJGGwHtzgrrtwqzdrtrngG
mllBlBZmMlQWRbQv
SGZBSFMZllJWmzvfpp
NTqbNrhHNHWgNqHrNhNQbbjHJLcnfnzLLnLmfcfccJcfQLcL
HggbTNRRTHqqbVSGMSZVWDMDwVPs
SBsSlvbPlFPvRlbPsMFZLgVLrLsJVgzrCJfVCH
jcNddNdGzZrVgNVJ
tTGwdcmWGdtwQmwmwZdwSlhBPbhPTBFRhlhSMFMR
RzStzTzzvvQvSHVvhVgBqMMFqhPM
ddlLLwNVLWLjbbLrjrbWrwmlhcFmBGgFMMPgBcGBBqPhggMs
dLwdVCVWWdfNwNwLrWrbfbJNptzDDHRnHptHtznHTppnQCtR
RzcfMBHLzpDQFmnDSWNB
dbqjtjVqJZZGjPGJCPGbPndNNDglrmQmNSDgSlSSng
hjCTqhCJbhVCGNvMcfvhfRLhvchz
sDDqDMtqshJhPvhhCpSCCWlZHSWp
bffRcbBGGTwGfGfbNjgSHZSgWwplHCClZZ
RTQBbcnbRNmGbGTQLbmbJVqLllsDVMsPDVVvttMd
nbLBjnqwgfRRBgBwnllbLlwScvPdZPcScZPcdFZJPvZPvcMZ
tChQpphHrrHztssZdcDJcPZcMvWv
hpTHVMQMtQtVpzBfwjfRnfwfnjVl
ljJlvvJQlrlcJcWpPzgthnPnzMgpgSpC
smtmZBmHZTVttHmqFqmzCSZSdndzShPNgPShgP
bVqFHLqLqfHHFwbBLHcwDQrDrtjlQvGjlRQQ
pwhVsPvVVCFtmhPhzqGqqZMZvGTTTMlGWM
drrrrDfDRrNQdQdrRrBdjGWqWqWlGlGtlGbGZGBTLc
tSDfgnHrdDtVSPSshJCSPh
WlWlDqhglLhsdgrcbFdJJpPpdBbB
ZQZvSvzRMSzjZjvZmMMpbFPQFVBrVbPcpbJFLB
SwGZmjvCRSMRjMzZvRnstHftNfswHsflLhNWHf
jsprCvGRQrtjCsQrGsrzvGHhgmHVmHZgggmMGVmhMbHm
FFFdDSdwSffJWqqMzzMmDVbZ
LLcdcfcfPwwBzdTTdtvlsrjCtvPvprnsjR
MvtSqNSWMzjwzFTD
ZRPlcRpQszNgszNwVT
bcZcrcPlcPLLLZllPlbcbLSBfWCvHvWWNSmSqNqfWN
rNdZpMGnddgggwHwzRPCzDDD
vcvhcTLhZLhLPCPHPDPPVvzH
LTmBmthWBchWLttttFJFLlFnGJNsfpdjNsnMnMpnpZdssn
ZHWFCvqBDdqqqCTDHHBWrgppTMhhVpspMPQcSgQVPS
jblbGffntRwltfMQVrrQscphfg
ztJrGtbwGztbmtzzRGnRznWNNCWmHHdFHdFNWWHHqCqZ
WGWSSZvVvqmrmzPm
NgjtwFFlwDsFghNsMtlcjljcPqrQHcZzQznpQQprnqqzHQ
tgMCwNhtgbdLZRbZCT
PQSPQrSGZnGnVFhpVhRRlvLvBDRV
tjctcjTMMpDTvFTlRD
JCftsccFCcmsJJGZGGmPHnQrGwGS
TrjRFFRnpnRCHNFSjSRrffJvJfzqQBsjqQqzzffd
ZtlgMDhZhgmGDLVZLlGtLPqdQQvvfBJJqzzBPdMzdd
VlLDgLLDWtGZwgtRNTNrFTqCwqHTrr
LpcDFDMMPjMLLjpcDGCHgHssGHWnbCBWBHvm
QfZhrhVVdZThlZlfVvVzZrTbgQnBHsCCHgJBsCsJBHmBmn
wwtvfZztlTVlhtrzzlLNpFFRjMPDpRcPFwRj
VzZhhQHQJJWJSSFWDGclbmNPgglPgVGc
ddBTqCjjBCcrqrCRrwGPGmmDGmbpBGNpNNgg
CRMjwsjwsLdLRrQFJSvMFMWZcHFW
JgJJPvtrhRPQQzSRMQFFSF
BLqsjsdLsMBqblnsGbBqVqdwSQSCSWwNFwczQWCNNwNCHn
ljqbpLbbdDlbDbqDDVtMttTTgpJJgThhJrJr
nflndmjbSnlTQGwvWGPHGRGj
NtstcMcDJMvwgHfFvDgR
qqqpLrMsLLqLNNnzbrdlbZSrznfz
ttZCCFjNjnPVCFQPPFbbStrzqzqrrrcwtmJJ
gTTMRMTWsTGGTddHTTbBzBLSmqbbJGzGmqqb
HpgpMTvRhHHTRDhMsHdHDRhjJlVPJjNFJnnFpQQVfPCjnP
VqJVQPpjQqPBbHwldmLfVVmd
tMGvrzzDGCDDddwLbgLvLwcm
TWDWCzTZDGMZtzWWtsFhbRRqRQRjhbNQBBTh
zgLgLHnnzCCvnsHSsZBZBsTRdD
rslllhJjcQNNGjpWJlSRTRdwBVSSNTPVSdPB
jGrGqjJfqccrfqGcGplrJpFvzggqmCtMzmsMnvMvvCgm"
        .to_string();
}
