// https://github.com/NixOS/nix/blob/f497711aa8b0deef6639a6b980a43ee6b7fff149/src/libutil/hash.cc
// static std::string printHash32(const Hash & hash)

pub fn nix32(hash: [u8; 32]) -> String {
    let nix32_chars = b"0123456789abcdfghijklmnpqrsvwxyz";

    let len = base32_len(32);
    let mut s = String::with_capacity(len);

    let mut n = len - 1;
    loop {
        let b = n * 5;
        let i = b / 8;
        let j = b % 8;

        let c = (hash[i] >> j)
            | (if i >= 32 - 1 {
                0
            } else {
                hash[i + 1].checked_shl((8 - j) as u32).unwrap_or_default()
            });
        s.push(*nix32_chars.get((c & 0x1f) as usize).unwrap() as char);
        if n == 0 {
            break;
        }
        n -= 1;
    }

    s
}

#[inline(always)]
const fn base32_len(hash_size: usize) -> usize {
    (hash_size * 8 - 1) / 5 + 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple() {
        let testcases = [
            (
                "bc66a48bbb482d6b3d89c118ddc9ebb44c809b5cdb596a8971bde35d97f0aa68",
                "0s5ay2bmvqxxf64nlnfvbjdq0k5lxg4xs661i4ynnba8pf5s8rmw",
            ),
            (
                "28e87802398096f53ceccae361dcc68a82da202a2473857ad0f6849b3b36ae7f",
                "0zxf6qxrp17ns1x8awr458hdm0laqvf63qyaxhygb5l07417is18",
            ),
            (
                "f3227d4cc9108c77011798da55181a0ef6b97d8ab215883c56190cc855c180d7",
                "1mw0q5awh30raqy8h5dji9yvkxhf38c5bnlq2w0pg30hr567s8pk",
            ),
            (
                "339d99f91242892d25a7761520db00d2f55d8b28f58b7c452f5c3a42f2c35ddf",
                "1psxqgr44fjw5x2pr2zm525mvxfj03dj05bnlwjjv2a22bwrk79k",
            ),
            (
                "61ca6dd5eb477e69b7b824989d3b357be57ce6d5779a80e7312fa28be9160500",
                "00052vlqp8ig67kq16kpspk7rrbv6lxrv614p2vnjzj7xganvjk1",
            ),
            (
                "ab799560df722f1eb864060f5355f804384fdf7b1b9a5474c8884c4530a5e424",
                "0974llq4ak48r1s596hvggglyf04z1am63q6cjw1wbvjvxh9aydb",
            ),
            (
                "04207c06a36ab4889c2628e018cc0d16b44324ca6c4707ea86aaaa204b2477f8",
                "1y3p4i5j1amahvm0fivcr8j47d0n1p61iq184sf8id3alc37q804",
            ),
            (
                "16d6f8385c2ba79f7346dc18c867b624bc6dcc8fcf7c2ec52c55b7ae3dc113f2",
                "1whkq4ysxdsm5k2jwz6giz66vg14nrkwh66w8rrrz9rbbhwgimhn",
            ),
            (
                "8fa3aa3896e1332d7bc307cd1c38c7f5f1fd86a189417d8a3780be18e6cbb3d4",
                "1m5krgk1igl06y57shc9l63gvwgmqww1rk87qdxjscz1jqwam8wg",
            ),
            (
                "b625b364b8f59a4870183cefa0954cc98333845526a40a42f0e9beca8d770701",
                "0087fy6wmgp9y110m916an2370y99jas1vrw31q4i6pmp1jb69dn",
            ),
            (
                "c362eaa590c9c95fc6223359ce1ebfa57776cf7ea1e47824f58d48dc16907088",
                "123hj0bdqj4dylj7ir51gv7pcxx5pwgcwn9k4b35zjf9j2jylqn3",
            ),
            (
                "cd3a7881cce9fd1de7b3eb42b80475d447b0c5d213ad4cf9b445d6edee2b5217",
                "05sj5gpfvmj5nkwlrb8ksb2v0iylfl2bhhpbngkivzg9rj0phfnd",
            ),
            (
                "b4cf398c21d054e8774af568395b0f7cd0e2b01322809c5dbd66c4135021ca57",
                "0mya45817i36pmfrr0122fqf5l3w1xdkjs7m99vyhm6h4663kkxl",
            ),
            (
                "9bdb563f9e164e58f3a922cef3d18dd87eebfa0161b937a7387011c0a2681579",
                "0y8md2ic04bh72kkgfb107xfnznqip8z7ki2m7rmhkhnkqzmdnwv",
            ),
            (
                "43e106d1811657af6b2f3371475144f8ef723d376ee641cf49d28d16a1e6a7e4",
                "1r57wshid3fj977l3rkf6wyp5vzq8i8lfw9k5xmsymqnh78hdqa3",
            ),
            (
                "fa84d8dce3dc1dbe92c2b4108565abcd79d5ec5e87beb073bc5d7eb5b18a6800",
                "0038iaqvazjxpirv1gl7bvndayfdmdjqa45lqa9bw7fwwgfdi17s",
            ),
        ];

        for (sha256, result) in testcases {
            let v = hex::decode(sha256).unwrap();
            let x = nix32(v.try_into().unwrap());
            assert_eq!(x, result);
        }
    }
}
