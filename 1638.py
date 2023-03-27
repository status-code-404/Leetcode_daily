def judge(x:str, y:str) -> bool:
    if x == y:
        return False

    diff = 0
    for a in zip(x,y):
        if a[0] != a[1]:
            if diff == 1:
                return False
            diff = 1
    return diff == 1

def countSubstrings(self, s: str, t: str) -> int:
    if t < s:
        s, t = t, s

    dict_ = dict()
    return_ = 0
    for i in range(len(s)):
        for k in range(i+1,len(s)+1):
            ss = s[i:k]
            tt = 0
            if dict_.get(ss) is not None:
                return_ += dict_.get(ss)
                continue
            len_ = k - i
            for j in range(len(t) - len_ + 1):
                if judge(ss, t[j:j+len_]):
                    tt += 1
            dict_[ss] = tt
            return_ += tt
    return return_
