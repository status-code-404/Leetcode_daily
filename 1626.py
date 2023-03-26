'''
假设你是球队的经理。对于即将到来的锦标赛，你想组合一支总体得分最高的球队。球队的得分是球队中所有球员的分数 总和 。

然而，球队中的矛盾会限制球员的发挥，所以必须选出一支 没有矛盾 的球队。如果一名年龄较小球员的分数 严格大于 一名年龄较大的球员，则存在矛盾。同龄球员之间不会发生矛盾。

给你两个列表 scores 和 ages，其中每组 scores[i] 和 ages[i] 表示第 i 名球员的分数和年龄。请你返回 所有可能的无矛盾球队中得分最高那支的分数 。



示例 1：

输入：scores = [1,3,5,10,15], ages = [1,2,3,4,5]
输出：34
解释：你可以选中所有球员。

示例 2：

输入：scores = [4,5,6,5], ages = [2,1,2,1]
输出：16
解释：最佳的选择是后 3 名球员。注意，你可以选中多个同龄球员。

示例 3：

输入：scores = [1,2,3,5], ages = [8,9,10,1]
输出：6
解释：最佳的选择是前 3 名球员。

'''

# 不知道咋优化，能跑过80%的用例剩下20%超时

dict_ = dict()

def get_max_score(people_list, score_split:int):
    global dict_
    if len(people_list) == 0:
        return 0

    this_people = people_list[0]
    if dict_.get((this_people, score_split)) is None:
        this_score = people_list[0][1]
        next_list = people_list[1:]

        if this_score < score_split:
            dict_[(this_people, score_split)] = get_max_score(next_list, score_split)
        else:
            dict_[(this_people, score_split)] = max(get_max_score(next_list, this_score) + this_score, get_max_score(next_list, score_split))

    return dict_[(this_people, score_split)]

class Solution:
    def bestTeamScore(self, scores: List[int], ages: List[int]) -> int:
        global dict_
        dict_.clear()
        people = sorted(zip(ages, scores))
        return get_max_score(people, -1)
