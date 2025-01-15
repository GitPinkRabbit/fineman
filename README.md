# Project of _Algorithm Analysis and Design_ (Fall 2024, IIIS, Tsinghua)

本 project 读一下 [_Single-Source Shortest Paths with Negative Real Weights in $\tilde O(m n^{8/9})$ Time_](https://doi.org/10.1145/3618260.3649614) by Jeremy T. Fineman (STOC ’24, Best Paper Award)。ArXiv 版本[点这里](https://arxiv.org/abs/2311.02520)。

尝试进行了代码实现：[GitHub/GitPinkRabbit/fineman](https://github.com/GitPinkRabbit/fineman)。

修改了原文一些（在我看来）不太清晰的逻辑，简化了部分证明过程。

最短路什么的废话就不多说了。

## 预处理

将图按如下步骤预处理：

1. 去除自环和重边，此步骤后图为简单图：
   - 即若有负自环则报告存在负环，否则去掉自环，重边保留权值最小的一条。
2. 若点 $u$ 的出边中有负边，设最小边权为 $w_0 < 0$：
   - 添加新点 $u'$，连 $u \to u'$，边权为 $w_0$。
   - 原有的所有边 $u \to v$ 权为 $w$，改为 $u' \to v$ 权为 $w - w_0$。

   此步骤后负边的端点两两不同，负边和负点一一对应。
3. 若有必要，添加不超过 $n$ 个新点和新边，使得每个点的入度与出度均不超过 $\lceil \frac{4 m}{n} \rceil + 1$（这里的 $n, m$ 指新图的点数与边数）。
   - 这总能做到，见代码 `impl From<SimpleGraph> for ProperGraph`。

## 定义

**$\boldsymbol{h}$ 跳路径**：定义一条路径 $u_0 \to u_1 \to \cdots \to u_k$ 的**跳数**为：

- 经过的负边数，即 $h = \lvert \{ i : w(u_i, u_{i + 1}) < 0 \} \rvert$。
- 定义两点 $(u, v)$ 间的 **$\boldsymbol{h}$ 跳距离** $\mathop{\text{dist}}\nolimits^h(u \rightsquigarrow v)$ 为它们之间不超过 $h$ 跳的路径的最短路径长度。若不存在则定义为 $+\infty$。
- 简单性质：
  - $\mathop{\text{dist}}\nolimits^h(u \rightsquigarrow v) < \mathop{\text{dist}}\nolimits^{h'}(u \rightsquigarrow v)$（ $h' < h$）。
  - 即 $\mathop{\text{dist}}\nolimits^h$ 随 $h$ 的增加而降低。
  - $\mathop{\text{dist}}\nolimits^h$ 不能为 $-\infty$（与之相对地， $\mathop{\text{dist}}\nolimits$ 可以为 $-\infty$，即当图中存在负环时）。
  - 若图中不存在负环，则 $\mathop{\text{dist}}\nolimits^h$ 随 $h$ 的增加而降低是有限度的，即当 $h$ 增加至两点间最短路的跳数后， $\mathop{\text{dist}}\nolimits^h$ 将不再变化。
- 对点集 $S, T$ 也可以定义 $\mathop{\text{dist}}\nolimits^h(S \rightsquigarrow v)$、 $\mathop{\text{dist}}\nolimits^h(u \rightsquigarrow T)$ 和 $\mathop{\text{dist}}\nolimits^h(S \rightsquigarrow T)$。性质类似。

**$\boldsymbol{h}$ 跳相关**：称 $(u, v)$ 是 **$\boldsymbol{h}$ 跳相关**的，当：

- $\mathop{\text{dist}}\nolimits^h(u \rightsquigarrow v) < 0$ 或 $\mathop{\text{dist}}\nolimits^h(v \rightsquigarrow u) < 0$。
- 即从 $u$ 出发经过一个负 $h$ 跳路径可达 $v$，或反之。

**负 $\boldsymbol{h}$ 跳可达范围**：定义一个点 $u$ 的**负 $\boldsymbol{h}$ 跳可达范围**为：

- 从 $u$ 出发经过一个负 $h$ 跳路径可达的点集。
- 进一步，定义一个点集 $S$ 的**负 $\boldsymbol{h}$ 跳可达范围**为\
  $$R^h(S) = \{ v \in V : \mathop{\text{dist}}\nolimits^h(S \rightsquigarrow v) < 0 \} \text{。}$$
- 简单性质：
  - $R^h(S) = \bigcup_{u \in S} R^h(u)$。
  - $R^h(S) \supseteq R^{h'}(S)$（ $h' < h$）。
  - $(u, v)$ 是 $h$ 跳相关的当且仅当 $v \in R^h(u)$ 或 $u \in R^h(v)$。

**$\boldsymbol{r}$ 遥远负点集/负边集/子图**：称一个负点集 $X \subseteq V^-$ 是 **$\boldsymbol{r}$ 遥远**的，当：

- 它的负 $r$ 跳可达范围的大小不超过 $n / r$，即 $\lvert R^r(X) \rvert \le n / r$。
- 进一步，这些负点所对应的负边形成的集合也称作 **$\boldsymbol{r}$ 遥远**的。
- 进一步，称由 $R^r(X)$ 中的点形成的导出子图为 **$\boldsymbol{r}$ 遥远子图**。
- 简单性质：
  - $r$ 遥远蕴含 $r'$ 遥远（ $r' < r$）。

**$\boldsymbol{1}$ 跳无关**：称一个点集 $I \subseteq V$（未必是负点的一个子集）是 **$\boldsymbol{1}$ 跳无关**的，当：

- 任意两个 $x, y \in I$ 都不是 $1$ 跳相关的。
- 即 $R^1(I) \cap I = \varnothing$。
- **注：原文定义要求 $I$ 是负点集。为了下文方便，我修改了定义。**

**$\boldsymbol{\beta}$ 距离**：对非负整数 $\beta \ge 0$，定义从点 $u$ 经过 $x$ 到 $v$ 的 **$\boldsymbol{\beta}$ 距离**为：

- $\mathop{\text{thru}}\nolimits^\beta(u \rightsquigarrow x \rightsquigarrow v) = \mathop{\text{dist}}\nolimits^\beta(u \rightsquigarrow x) + \mathop{\text{dist}}\nolimits^\beta(x \rightsquigarrow v)$。
- 若 $\mathop{\text{thru}}\nolimits^\beta(u \rightsquigarrow x \rightsquigarrow v) < 0$，则称 $x$ 是 **$\boldsymbol{\beta}$ 介于** $u \rightsquigarrow v$ **之间**的。
- 称 $u \rightsquigarrow v$ 的 **$\boldsymbol{\beta}$ 间性** $\mathop{\text{BW}}\nolimits^\beta(u \rightsquigarrow v)$ 为 $\beta$ 介于 $u \rightsquigarrow v$ 之间的点数，即 $\mathop{\text{BW}}\nolimits^\beta(u \rightsquigarrow v) = \lvert \{ x \in V : \mathop{\text{thru}}\nolimits^\beta(u \rightsquigarrow x \rightsquigarrow v) < 0 \} \rvert$。

**负夹心面包**：定义**负夹心面包**为满足如下条件的三元组 $(x, U, y)$：

- $U \subseteq V^-$， $x, y \in V$。
- $\mathop{\text{dist}}\nolimits^1(x \rightsquigarrow u) < 0$ 对所有 $u \in U$。
- $\mathop{\text{dist}}\nolimits^1(u \rightsquigarrow y) < 0$ 对所有 $u \in U$。
- 称该负夹心面包的大小为 $\lvert U \rvert$。

## 算法

Johnson 技巧 / pricing / 点势函数：

- 函数 $\varphi \colon V \to \R$，定义新边权 $w_{\varphi}(u, v) = w(u, v) + \varphi(u) - \varphi(v)$。
- 新图中的最短路保持不变，即 $\mathop{\text{dist}}\nolimits_\varphi(u \rightsquigarrow v) = \mathop{\text{dist}}\nolimits(u \rightsquigarrow v) + \varphi(u) - \varphi(v)$，或等价地， $\mathop{\text{dist}}\nolimits(u \rightsquigarrow v) = \mathop{\text{dist}}\nolimits_\varphi(u \rightsquigarrow v) - \varphi(u) + \varphi(v)$。
- 对于任意的 $\varphi$，需要保证它不会将非负边转为负边，即 $w_{\varphi}(u, v) \ge 0$ 对所有 $(u, v) \in E^+$。
- 如果同时对一条负边 $(u, v)$ 有 $w(u, v) + \varphi(u) - \varphi(v) \ge 0$，就相当于消除了一条负边。
- 这等价于 $\varphi(v) \le \varphi(u) + w(u, v)$。
- 即如果令 $\varphi(u)$ 为图的距离函数，就能消除图中的所有负边。
- 如果图中有 $k$ 条负边，通过运行 $k + 1$ 次 Dijkstra 算法，可以求出距离函数，时间复杂度为 $O(k (m + n \log n))$。
- 这个技巧也可以对一个子图使用：一个保留所有非负边，并仅保留部分负边 $N \subseteq E^-$ 的子图 $G_N = (V, E^+ \cup N, w)$。它的距离函数可以消除所有 $N$ 中的负边，而对其余负边 $E^- \setminus N$ 则不作保证。
- 通过平凡地运用这个技巧，每条负边的消除均摊地需要 $O(m + n \log n)$ 的时间。（显然需要改进。）

$r$ 跳削减图：

- 现有一张图 $G = (V, E^+ \cup E^-, w)$，目标是构造一张图 $H = (V_H, E_H, w_H)$，使得 $V \subseteq V_H$ 且：
  - 在图 $H$ 中， $\mathop{\text{dist}}\nolimits_H(u \rightsquigarrow v) = \mathop{\text{dist}}\nolimits(u \rightsquigarrow v)$（对 $u, v \in V$），且一条 $G$ 中的 $h$ 跳路径对应一条 $H$ 中的 $\lceil h / r \rceil$ 跳路径。
  - 这里假设 $1 \le r \le \lvert E^- \rvert$。
- 称这样的 $H$ 为 $G$ 的 **$\boldsymbol{r}$ 跳削减图**。
- 即，在 $H$ 中，为求得距离函数，所需的 Dijkstra 趟数减少了：从原来的 $k + 1$ 趟（ $k = \lvert E^- \rvert$）变为 $\lceil k / r \rceil + 1$ 趟。
- 一个一般的构造如下：
  - 将每个点复制 $r + 1$ 份，将点 $u$ 的复制称作 $u_0, \ldots, u_r$。把 $u$ 识别为 $u_0$。
  - 对每条 $G$ 中的非负边 $(u, v)$，连接同一层中的 $(u_i, v_i)$（对每一层 $0 \le i \le r$）。
  - 对每条 $G$ 中的负边 $(u, v)$，连接每一层中的 $u_i$ 与下一层中的 $v_{i + 1}$，即对除了第 $r$ 层的每一层 $0 \le i < r$，连接 $(u_i, v_{i + 1})$。
  - 对除了第 $0$ 层的每一层 $1 \le i \le r$，连接 $(u_i, u_0)$。
  - 这些 $H$ 中的边的边权如何选择呢？我们希望在 $G$ 中每经过 $r$ 条负边，就对应在 $H$ 中只需经过一条负边。
  - 即这 $r$ 条负边对应着逐渐增加的层号，从 $0$ 到 $r$，最后通过层号下降的边 $(u_i, u_0)$ 回到第 $0$ 层。
  - 仍然使用 Johnson 技巧，令点 $u_i$ 的势为 $\varphi(u_i) = \mathop{\text{dist}}\nolimits^i(V \rightsquigarrow u)$。
  - 边权为 $w_H(u_i, v_j) = w(u, v) + \mathop{\text{dist}}\nolimits^i(V \rightsquigarrow u) - \mathop{\text{dist}}\nolimits^j(V \rightsquigarrow v)$。
  - 则根据 $\mathop{\text{dist}}\nolimits^{i + [w(u, v) < 0]}(V \rightsquigarrow v) \le \mathop{\text{dist}}\nolimits^i(V \rightsquigarrow u) + w(u, v)$，除了 $(u_i, u_0)$ 这些边，其他边权都 $\ge 0$。
  - 注：由于 $\mathop{\text{dist}}\nolimits^0(V \rightsquigarrow u) = 0$ 恒成立，有 $\varphi$ 在第 $0$ 层上恒为 $0$，故不影响 $\mathop{\text{dist}}\nolimits_H(u \rightsquigarrow v) = \mathop{\text{dist}}\nolimits(u \rightsquigarrow v)$ 成立。
- 根据如上构造， $H$ 拥有 $O(r n)$ 个点， $O(r m)$ 条边。
- 故 $\lceil k / r \rceil + 1$ 趟 Dijkstra 仍然需要 $O((k / r) (r m + r n \log r n)) = O(k (m + n \log n))$ 的时间，并未改变。

消除 $r$ 遥远负边：

- 如果负边集 $N$ 是 $r$ 遥远的，我们将看到，对上述 $r$ 跳削减图的更优构造大有裨益。
- 设图 $G = (V^+ \cup V^-, E^+ \cup E^-, w)$，以及 $N \subseteq E^-$ 是负边集， $X \subseteq V^-$ 是对应的负点集。
- 忽略其余负边，得到图 $G_N = (V, E^+ \cup N, w)$。
- 则在 $G_N$ 中，显然 $R^r(X)$ 只会更小，因为负边越来越少了。
- 而图的总点数没有减少，故 $X$ 仍是 $r$ 遥远负点集， $N$ 仍是 $r$ 遥远负边集。
- 此时，在图 $G_N$ 中，如果一个点 $u$ 在 $r$ 遥远子图之外，即 $\mathop{\text{dist}}\nolimits_N^r(X \rightsquigarrow u) \ge 0$，则必有 $\mathop{\text{dist}}\nolimits_N^r(V \rightsquigarrow u) \ge 0$，因为从其他点（非负点）出发并不比从负点出发更优。又由于 $\mathop{\text{dist}}\nolimits_N^i(V \rightsquigarrow u)$ 必然 $\le 0$，因为空路径的权值为 $0$。我们有 $\mathop{\text{dist}}\nolimits_N^r(V \rightsquigarrow u) = 0$，因此每个 $\mathop{\text{dist}}\nolimits_N^i(V \rightsquigarrow u)$ 都 $= 0$（对每个 $0 \le i \le r$）。
- 这就是说，对在 $r$ 遥远子图之外的点 $u$，有：
  - $w_H(u_i, v_j) = w(u, v) - \mathop{\text{dist}}\nolimits_N^j(V \rightsquigarrow v)$；
  - $w_H(v_i, u_j) = w(v, u) + \mathop{\text{dist}}\nolimits_N^i(V \rightsquigarrow v)$。
- 即与所有 $u_i$ 相连的边的边权不取决于所在层数 $i$。故 $r + 1$ 个点 $u_0, \ldots, u_r$ 可以缩为一个点 $u_0 = u$。
- 这样一来， $H$ 的点数就变成 $n + r \lvert R^r(X) \rvert$，边数就变成 $O(m + (m / n) r \lvert R^r(X) \rvert)$（这里利用了每个点的度数为 $O(m / n)$ 的性质）。
- 根据 $r$ 遥远子图的定义，有 $\lvert R^r(X) \rvert \le n / r$。故上述界改为点数 $O(n)$，边数 $O(m)$。
- 即如果 $N$ 是 $r$ 遥远的， $r$ 跳削减图的大小可与原图同阶。
- 构造上述 $r$ 跳削减图仍需要计算 $\mathop{\text{dist}}\nolimits^i(V \rightsquigarrow u)$（ $0 \le i \le r$），故需要 $O(r (m + n \log n))$ 时间。
- 对 $r$ 跳削减图求距离函数需要 $\lceil k / r \rceil + 1$ 遍 Dijkstra，这需要 $O((k / r) (m + n \log n))$ 时间。
- 故加总得 $O((r + k / r) (m + n \log n))$ 时间。

消除 $1$ 跳无关负边：

- 如果负点集 $I \subseteq V^-$ 是 $1$ 跳无关的，有如下简单方式可以消除它们对应的负边 $N$。
- 忽略其余负边，得到图 $G_N = (V, E^+ \cup N, w)$。
- 使用势函数 $\varphi(u) = \mathop{\text{dist}}\nolimits_N^1(V \rightsquigarrow u)$。
- 此时对负边 $(u, v)$ 有新边权 $w_\varphi(u, v) = w(u, v) + \mathop{\text{dist}}\nolimits_N^1(V \rightsquigarrow u) - \mathop{\text{dist}}\nolimits_N^1(V \rightsquigarrow v)$。
- 由于 $I$ 是 $1$ 跳无关的，可知 $\mathop{\text{dist}}\nolimits_N^1(I \rightsquigarrow u) \ge 0$，这说明 $\mathop{\text{dist}}\nolimits_N^1(V \rightsquigarrow u) = 0$。
- 而 $\mathop{\text{dist}}\nolimits_N^1(V \rightsquigarrow v) \le w(u, v)$，故新边权 $w_\varphi(u, v) \ge 0$。
- 对于非负边 $(u, v)$，由于 $\mathop{\text{dist}}\nolimits_N^1(V \rightsquigarrow v) \le \mathop{\text{dist}}\nolimits_N^1(V \rightsquigarrow u) + w(u, v)$，显然有新边权 $\ge 0$。
- 时间复杂度为 $O(m + n \log n)$。

如果能得到足够大的 $r$ 遥远负边集，或足够大的 $1$ 跳无关负边集，我们的算法就有着落了。

然而，无论如何这都牵扯到与负 $h$ 跳可达范围 $R^h(u)$ 有关的定义，而该集合是取决于势函数的。

然而是否真的存在这样的势函数，使得要么足够大的 $r$ 遥远负边集存在，要么足够大的 $1$ 跳无关负边集存在，是存疑的。

我们将看到，如果能使每对点 $u \rightsquigarrow v$ 的 $\beta$ 间性都足够小，将对算法大有帮助。

现在的目标是，固定常数 $\beta$ 和 $\tau$，并找到一个势函数使得 $\mathop{\text{BW}}\nolimits^\beta(u \rightsquigarrow v) \le n / \tau$ 对所有 $u \rightsquigarrow v$ 都成立。

算法如下：

- 均匀随机 $\Theta(\tau \log n)$ 个特殊点。
- 找到一个势函数 $\varphi$，使得所有从特殊点出发或到达特殊点的 $\beta$ 跳距离都非负。（或判断图中有负环。）
- 先不管如何找到该势函数。现在的问题是，经过这样的势函数后，间性是否足够小。
- 一个显然的观察是，特殊点不可能成为 $\beta$ 介于其他点之间的点。
  - 因为与特殊点有关的 $\beta$ 跳距离都非负，经过特殊点的 $\beta$ 距离也非负。
- 但仅这样只能让间性的上界为 $n - \Theta(\tau \log n)$。
- 注意 $\mathop{\text{thru}}\nolimits_\varphi^\beta(u \rightsquigarrow x \rightsquigarrow v) = \mathop{\text{thru}}\nolimits^\beta(u \rightsquigarrow x \rightsquigarrow v) + \varphi(u) - \varphi(v)$，不取决于 $\varphi(x)$。
- 故如果对某个点 $y$ 来说有 $\mathop{\text{thru}}\nolimits_\varphi^\beta < 0$，则考虑原 $\mathop{\text{thru}}\nolimits^\beta$，它必然是所有 $\mathop{\text{thru}}\nolimits^\beta$ 中比较小的。
- 由于特殊点是随机选取的，而 $\mathop{\text{thru}}\nolimits^\beta$ 只要比某个特殊点 $x$ 的 $\mathop{\text{thru}}\nolimits^\beta$ 大，就有 $\mathop{\text{thru}}\nolimits_\varphi^\beta \ge 0$。
- 对于一个特定的 $u \rightsquigarrow v$， $\mathop{\text{BW}}\nolimits^\beta(u \rightsquigarrow v) > n / \tau$ 的概率有上界：
  - $[1, n]$ 中均匀随机选 $T = \Theta(\tau \log n)$ 个数，最小的数的编号大于 $n / \tau$ 的概率。
  - 即\
    $$\prod_{i = 0}^{T - 1} \frac{n - n / \tau - i}{n - i} \le \prod_{i = 0}^{T - 1} \frac{n - n / \tau}{n} = \Bigl( 1 - \frac{1}{\tau} \Bigr)^T = \biggl( \Bigl( 1 - \frac{1}{\tau} \Bigr)^{\tau} \biggr)^{\Theta(\log n)} \le \frac{1}{n^{\Theta(1)}} \text{。}$$
- 再根据事件的并的概率不超过各事件概率之和，有该势函数满足每对点的条件的概率不小于 $1 - 1 / n^{\Theta(1) - 2}$。
- 现在的问题是，如何找到这样的势函数。（使得所有从特殊点出发或到达特殊点的 $\beta$ 跳距离都非负。）
  - 设特殊点集为 $T$。
  - 对每个 $x \in T$，求所有 $v \in V$ 的 $\mathop{\text{dist}}\nolimits^\beta(x \rightsquigarrow v)$ 和 $\mathop{\text{dist}}\nolimits^\beta(v \rightsquigarrow x)$。这一步需要 $O(\beta \lvert T \rvert (m + n \log n))$ 时间。
  - 建新图 $H = (V, E_H, w_H)$，其中 $E_H$ 包含有 $\lvert E^+ \rvert + 2 \lvert T \rvert n$ 条边，具体地：
    - 包含原图的所有非负边。（**注：原文不含此步。**）
    - 每对 $x \to v$ 和 $v \to x$ 都有一条边，边权相应地为 $\mathop{\text{dist}}\nolimits^\beta(x \rightsquigarrow v)$ 和 $\mathop{\text{dist}}\nolimits^\beta(v \rightsquigarrow x)$。
  - 断言：该图若不存在负环则最短路的跳数不超过 $2 \lvert T \rvert$。
    - 这是因为只有那些与特殊点相关的边才可能是负边，而一条最短路最多经过 $\lvert T \rvert$ 个特殊点，故最多经过 $2 \lvert T \rvert$ 条负边。
  - 故进行 $2 \lvert T \rvert + 1$ 趟 Dijkstra，时间复杂度为 $O(\lvert T \rvert (m + \lvert T \rvert n + n \log n)) = O(\lvert T \rvert m + \lvert T \rvert^2 n)$，即可求出该图的距离函数。
    - 否则该图存在负环，这同时也说明原图存在负环，毕竟新图的边对应原图的路径。
  - 断言：该距离函数作为原图上的势函数，是合法的，且满足所有从特殊点出发或到达特殊点的 $\beta$ 跳距离都非负。
    - 对于合法性，由于 $H$ 包含所有原图的非负边，故距离函数不会把非负边转为负边。（**注：原文由于不含原图的边，使用了更复杂的论证（我无法确认正确性）。这就是为什么我修改了定义，因为这样证明起来更简单。**）
    - 对于后者，因为势函数不改变最短路本身，该距离函数把新图的边权变为非负，也即把原图的 $x \rightsquigarrow v$ 和 $v \rightsquigarrow x$ 的 $\beta$ 跳最短路变为非负，即 $\beta$ 跳距离非负。
- 此算法时间复杂度为 $O(\beta \lvert T \rvert (m + n \log n) + \lvert T \rvert^2 n) = O(\beta \tau (m + n \log n) \log n + \tau^2 n \log^2 n)$。

现在我们得到了一张间性足够小（ $\le n / \tau$）的图，接下来的问题是如何基于此找到足够大的 $r$ 遥远负边集或 $1$ 跳无关负边集。

下面我们展示如何通过负夹心面包找到一个足够大的 $r$ 遥远负边集：

- 对一个负夹心面包 $(x, U, y)$ 和一个跳数 $\beta \ge 1$，考虑使用势函数\
  $$\varphi(v) = \min(0, \max(\mathop{\text{dist}}\nolimits^\beta(x \rightsquigarrow v), -\mathop{\text{dist}}\nolimits^\beta(v \rightsquigarrow y))) \text{。}$$
  这个势函数在 $O(\beta (m + n \log n))$ 时间内可以计算出来。
- 这样的势函数满足：
  - 它是合法的，即不会把非负边转为负边。
    - 只需证明，对于非负边 $(u, v)$，有 $\varphi(v) \le \varphi(u) + w(u, v)$。
    - 由 $\mathop{\text{dist}}\nolimits^\beta(x \rightsquigarrow v) \le \mathop{\text{dist}}\nolimits^\beta(x \rightsquigarrow u) + w(u, v)$ 和 $\mathop{\text{dist}}\nolimits^\beta(u \rightsquigarrow y) \le w(u, v) + \mathop{\text{dist}}\nolimits^\beta(v \rightsquigarrow y)$：\
      $$\begin{aligned} \varphi(v) &= \min(0, \max(\mathop{\text{dist}}\nolimits^\beta(x \rightsquigarrow v), -\mathop{\text{dist}}\nolimits^\beta(v \rightsquigarrow y))) \\ &\le \min(0, \max(\mathop{\text{dist}}\nolimits^\beta(x \rightsquigarrow u) + w(u, v), -\mathop{\text{dist}}\nolimits^\beta(u \rightsquigarrow y) + w(u, v))) \\ &= \min(-w(u, v), \max(\mathop{\text{dist}}\nolimits^\beta(x \rightsquigarrow u), -\mathop{\text{dist}}\nolimits^\beta(u \rightsquigarrow y))) + w(u, v) \\ &\le \min(0, \max(\mathop{\text{dist}}\nolimits^\beta(x \rightsquigarrow u), -\mathop{\text{dist}}\nolimits^\beta(u \rightsquigarrow y))) + w(u, v) \\ &= \varphi(u) + w(u, v) \text{。} \end{aligned}$$
  - $\varphi(u) = 0$ 对所有 $u \in U$。
    - 因为 $\mathop{\text{dist}}\nolimits^\beta(u \rightsquigarrow y) < 0$，故取负后 $> 0$，取 $\max$ 后仍 $> 0$，再取 $\min$ 后 $= 0$。
  - 对“大部分”其他点， $\varphi(v) \le \min(0, \mathop{\text{dist}}\nolimits^\beta(x \rightsquigarrow v))$。
    - 若 $v$ **不** $\beta$ 介于 $x \rightsquigarrow y$ 之间，有 $\mathop{\text{dist}}\nolimits^\beta(x \rightsquigarrow v) + \mathop{\text{dist}}\nolimits^\beta(v \rightsquigarrow y) \ge 0$。
    - 也即 $\mathop{\text{dist}}\nolimits^\beta(x \rightsquigarrow v) \ge -\mathop{\text{dist}}\nolimits^\beta(v \rightsquigarrow y)$。
    - 故 $\varphi(v) = \min(0, \mathop{\text{dist}}\nolimits^\beta(x \rightsquigarrow v))$。
    - 即只有 $1 / \tau$ 比例的点不满足这个条件。
- 现考虑 $R_\varphi^{\beta - 1}(U)$。对“大部分”其他点 $v$，有 $\mathop{\text{dist}}\nolimits_\varphi^{\beta - 1}(U \rightsquigarrow v) > 0$：
  - 根据上述，有 $\varphi(v) \le \min(0, \mathop{\text{dist}}\nolimits^\beta(x \rightsquigarrow v))$。
  - 又对任意 $u \in U$ 有 $\mathop{\text{dist}}\nolimits^\beta(x \rightsquigarrow v) \le \mathop{\text{dist}}\nolimits^1(x \rightsquigarrow u) + \mathop{\text{dist}}\nolimits^{\beta - 1}(u \rightsquigarrow v)$。
  - 然而，根据负夹心面包的定义，有 $\mathop{\text{dist}}\nolimits^1(x \rightsquigarrow u) < 0$，故 $\mathop{\text{dist}}\nolimits^\beta(x \rightsquigarrow v) < \mathop{\text{dist}}\nolimits^{\beta - 1}(u \rightsquigarrow v)$。
  - 然后是\
    $$\begin{aligned} \mathop{\text{dist}}\nolimits_\varphi^{\beta - 1}(u \rightsquigarrow v) &= \mathop{\text{dist}}\nolimits^{\beta - 1}(u \rightsquigarrow v) + \varphi(u) - \varphi(v) \\ &> \mathop{\text{dist}}\nolimits^\beta(x \rightsquigarrow v) + 0 - \min(0, \mathop{\text{dist}}\nolimits^\beta(x \rightsquigarrow v)) \\ &= \max(0, \mathop{\text{dist}}\nolimits^\beta(x \rightsquigarrow v)) \\ &\ge 0 \text{。} \end{aligned}$$
  - 故对“大部分”其他点 $v$，有 $\mathop{\text{dist}}\nolimits_\varphi^{\beta - 1}(U \rightsquigarrow v) > 0$。
- 即 $R_\varphi^{\beta - 1}(U)$ 中只有可能包含 $\beta$ 介于 $x \rightsquigarrow y$ 之间的点，故 $\lvert R_\varphi^{\beta - 1}(U) \rvert \le n / \tau$。（对于 $U$，实际上有 $U$ 本身就 $\beta$ 介于 $x \rightsquigarrow y$ 之间。考察负夹心面包的定义即可知。）
- 这就说明，在间性足够小的图中，负夹心面包的 $U$ 就给出了一个 $\min(\tau, \beta - 1)$ 遥远负边集。

由上可知，在拥有一张间性足够小的图的基础上，只要找到一个足够大的负夹心面包，就能得到一个足够大的 $r$ 遥远负边集。

现在的问题是，如何找到这样的负夹心面包，或者找到一个足够大的 $1$ 跳无关负边集。

接下来的算法将解决最后的问题：

- 给定图 $G = (V^+ \cup V^-, E^+ \cup E^-, w)$，和一个点集 $U_0 \subseteq V$（未必是负点的一个子集）。
- 设 $\hat k = \lvert U_0 \rvert$。再给定一个整数参数 $\rho \in [1, \hat k]$。有如下算法存在：
  - 在期望 $O((m + n \log n) \log n)$ 的时间内，得到以下三种结果之一：
    1. 汇报图中有负环；
    2. 找到一个大小为 $\Omega(\hat k / \rho)$ 的点集 $U \subseteq U_0$ 和一个点 $y \subseteq U_0$，满足 $\mathop{\text{dist}}\nolimits^1(u \rightsquigarrow y) < 0$ 对所有 $u \in U$；
    3. 找到一个大小为 $\Omega(\rho)$ 的点集 $I \subseteq U_0$，满足 $I$ 是 $1$ 跳无关的（ $I$ 未必是负点的一个子集）。
  - **注：这里我修改了原文的算法的定义，使其不要求 $\boldsymbol{U_0}$ 是负点的一个子集。**
- 不难看出，有了上述算法，并令 $U_0$ 为负点的一个子集，相当于每次可以找到“一半”的负夹心面包。所以有必要运行两趟上述算法，平衡指数可得：
  - 设 $\lvert V^- \rvert = k$。
  - 第一遍，令 $U_0 = V^-$， $\rho = k^{1/3}$：
    - 要么得到一个 $1$ 跳无关负点集 $I_1$，大小为 $\Omega(k^{1/3})$；
    - 要么得到一个大小为 $\Omega(k^{2/3})$ 的负点集 $U_1$ 和一个点 $y$，满足 $\mathop{\text{dist}}\nolimits^1(u \rightsquigarrow y) < 0$ 对所有 $u \in U_1$。
    - 若得到 $I_1$，则算法结束。
  - 第二遍，令 $U_0 = U_1$， $\rho = k^{1/3}$，但在反向图上运行：
    - 这里需要注意，反向图上的负点并不是原来的负点，而是原来的负边的终点。
    - 所以令 $U_0 = U_1$ 时 $U_0$ 在反向图上并不是负点的一个子集，这就是为什么我修改了算法的定义。
    - 要么得到一个 $1$ 跳无关点集 $I_2$，大小为 $\Omega(k^{1/3})$：
      - 注：这里的 $I_2$ 不是反向图上的负点集，但确实是原图上的负点集。
      - 在反向图上， $1$ 跳无关的描述为：对任意 $x, y \in I_2$ 有 $(\mathop{\text{dist}}\nolimits^1)^T(x \rightsquigarrow y) \ge 0$，注意上标 $T$ 表示反向图。
      - 在原图中，这就等价于 $\mathop{\text{dist}}\nolimits^1(y \rightsquigarrow x) \ge 0$。
      - 根据对称性可知， $I_2$ 是原图上的 $1$ 跳无关负点集。
    - 要么得到一个大小为 $\Omega(k^{1/3})$ 的点集 $U_2 \subseteq U_1$ 和一个点 $x$，满足 $(\mathop{\text{dist}}\nolimits^1)^T(u \rightsquigarrow x) < 0$ 对所有 $u \in U_2$：
      - 同样地，这里的 $U_2$ 不是反向图上的负点集，但确实是原图上的负点集。
      - 在原图中， $(\mathop{\text{dist}}\nolimits^1)^T(u \rightsquigarrow x) < 0$ 就等价于 $\mathop{\text{dist}}\nolimits^1(x \rightsquigarrow u) < 0$。
      - 结合 $\mathop{\text{dist}}\nolimits^1(u \rightsquigarrow y) < 0$ 对所有 $u \in U_2 \subseteq U_1$，可知 $(x, U_2, y)$ 是一个负夹心面包。
- 接下来我们将展示如何实现上述算法。
- 首先令记号 $C(U_0, v) = \lvert \{ u \in U_0 : \mathop{\text{dist}}\nolimits^1(u \rightsquigarrow v) < 0 \} \rvert$ 表示 $U_0$ 中负 $1$ 跳可达 $v$ 的点数。
- 我们要做的是对所有 $v \in U_0$ 估测 $C(U_0, v)$ 的大小。
- 将 $U_0$ 分为两部分 $H, L$，称为重点集和轻点集，满足：
  - $C(U_0, v) = \Omega(\hat k / \rho)$ 对所有 $v \in H$；
  - $C(U_0, v) = O(\hat k / \rho)$ 对所有 $v \in L$。
  - 这可以通过在 $U_0$ 中以 $p = \rho / \hat k$ 的概率随机采样得到集合 $U'$，然后计算 $R^1(U')$。
  - 将在 $R^1(U')$ 中的点放入 $H$，其余放入 $L$。
  - （为了保证概率，需重复 $\Theta(\log n)$ 次，然后 Chernoff–Hoeffding 定理将说明概率的正确性。）
  - 具体来说：
  - 精确地，定义重点为 $C(U_0, v) \ge 2 \hat k / \rho$ 的点，轻点为 $C(U_0, v) \le (1 / 8) \hat k / \rho$ 的点。
  - 可能存在其他点不属于重点或轻点。
  - 我们希望找到集合 $H, L$，使得所有重点都在 $H$ 中，所有轻点都在 $L$ 中。（其他点不在乎。）
  - 为此，考虑如上采样得到的集合 $U'$ 和 $R^1(U')$：
    - 一个点 $v$ 在 $R^1(U')$ 中，等价于采样时采到了 $v$ 对应的那 $C(U_0, v)$ 个点中的至少一个。
    - 概率即为 $1 - (1 - p)^{C(U_0, v)}$。
    - 对于重点，这个概率就 $\ge 1 - (1 - \rho / \hat k)^{2 \hat k / \rho} \ge 1 - 1 / \mathrm{e}^2 > 6 / 7$。
    - 对于轻点，这个概率就 $\le 1 - (1 - \rho / \hat k)^{(1 / 8) \hat k / \rho} \le 1 - (1 - 1 / 8) = 1 / 8$。
  - 即重点在 $R^1(U')$ 中的概率 $q_\mathrm{H} \ge 6 / 7$，轻点在 $R^1(U')$ 中的概率 $q_\mathrm{L} \le 1 / 8$。
  - 且上述事件，固定一个重点或轻点后，在进行独立的对 $U'$ 的采样时，是独立的。
  - 故根据 Chernoff–Hoeffding 定理，重复 $\lceil c \ln n \rceil$ 次采样后，对固定的一个重点或轻点，它在 $R^1(U')$ 中的次数 $\ge \lceil c \ln n \rceil / 2$ 的概率有计算：
    - 对重点， $\ge 1 - \Pr(X \le (1 / 2) \lceil c \ln n \rceil) \ge 1 - \Pr(X \le \mu - (q_\mathrm{H} - 1 / 2) \lceil c \ln n \rceil) \ge 1 - \biggl( \Bigl( \dfrac{q_\mathrm{H}}{1 / 2} \Bigr)^{1 / 2} \Bigl( \dfrac{1 - q_\mathrm{H}}{1 / 2} \Bigr)^{1 / 2} \biggr)^{\lceil c \ln n \rceil} \ge 1 - ((1 / \mathrm{e})^{1 / 3})^{\lceil c \ln n \rceil} \ge 1 - 1 / n^{c / 3}$。
    - 对轻点，类似地， $\le 1 / n^{c / 3}$。
  - 再根据事件的并的概率不超过各事件概率之和，有所有重点都在 $H$ 中，所有轻点都在 $L$ 中的概率 $\ge 1 - 1 / n^{c / 3 - 1}$。
  - 上述算法的时间复杂度为 $O((m + n \log n) \log n)$，因为需要重复 $\Theta(\log n)$ 次采样和计算，而每次计算 $R^1(U')$ 的时间为 $O(m + n \log n)$。
- 得到了 $H, L$ 两个集合后（假设确实满足所有重点都在 $H$ 中、所有轻点都在 $L$ 中），我们分类讨论：
  - 若 $H \ne \varnothing$，即存在非轻点：
    - 在 $H$ 中任取 $y \in H$，有 $C(U_0, y) > (1 / 8) \hat k / \rho$。
    - 令 $U = \{ u \in U_0 : \mathop{\text{dist}}\nolimits^1(u \rightsquigarrow y) < 0 \}$。
    - 检查是否有 $\lvert U \rvert \ge (1 / 8) \hat k / \rho$，若不然，说明 $H, L$ 的划分不正确，重启算法。
    - 若是，输出 $U, y$。
    - 求出 $U$ 的时间复杂度为 $O(m + n \log n)$。
  - 若 $H = \varnothing$，即不存在重点：
    - 令 $I'$ 为 $U_0$ 中大小为 $\lceil \rho / 4 \rceil$ 的随机子集。
    - 令 $I = I' \setminus R^1(I')$，此时必有 $I$ 是 $1$ 跳无关的。
    - 现在我们需要证明 $I$ 足够大。
    - $I'$ 中的一个点 $v$ **被其他点删去**的概率为：
      - 注：这里**被其他点删去**指的是 $v \in R^1(u)$ 对某个 $u \in I'$ 且 $u \ne v$。
        - 相对地， $v$ 可能被自己删去，即 $v \in R^1(v)$，即 $\mathop{\text{dist}}\nolimits^1(v \rightsquigarrow v) < 0$，然而这意味着存在负环。
      - 所有可能删去 $v$ 的 $u$ 的数量不超过 $C(U_0, v)$（可能不取等号，因为可能要扣掉 $v$ 自己）。
      - $I'$ 正好采样到它们中的至少一个的概率（条件概率，因为已知 $v \in I'$）不超过 $C(U_0, v) \frac{\lceil \rho / 4 \rceil - 1}{\hat k - 1}$。
      - 由于不存在重点，有 $C(U_0, v) < 2 \hat k / \rho$。
      - 故概率不超过 $(2 \hat k / \rho) \frac{\lceil \rho / 4 \rceil - 1}{\hat k - 1} \le \frac{\hat k / 2}{\hat k - 1}$。
      - 假设 $\hat k \ge 5$，概率就不超过 $5 / 8$。
    - 故**因其他点而被删去**的总点数期望不超过 $(5 / 8) \lvert I' \rvert$。
    - 根据 Markov 不等式，**因其他点而被删去**的总点数不超过 $(3 / 4) \lvert I' \rvert$ 的概率至少为 $1 - \dfrac{5 / 8}{3 / 4} = \dfrac{1}{6}$。
    - 即**被留下**或**仅因自己而被删去**的点至少有 $\rho / 16$ 个，的概率至少为 $1 / 6$。
    - 即，在不存在**仅因自己而被删去**的点的前提下， $I$ 至少有 $\rho / 16$ 个点的概率至少为 $1 / 6$。
    - 那么，如何探测**仅因自己而被删去**的点呢？
      - 具体来说：只需跟踪每个点最短路的来源：如果 $v$ 仅因自己而被删去，那么它的距离是负数，且最短路的来源必然是它自己。
      - 同时，如果发现有的点的距离是负数且最短路的来源是它自己，就说明存在负环，可以直接结束算法。
    - 故，先确认不存在来源是自己且距离为负的点，就相当于确认了不存在**仅因自己而被删去**的点。然后 $I$ 至少有 $\rho / 16$ 个点的概率就有保证了。
    - 故进行 $\Theta(\log n)$ 次（具体地， $\lceil c' \ln n \rceil$ 次）对 $I'$ 的采样，就能将找到满足大小 $\ge \rho / 16$ 的 $1$ 跳无关的 $I$ 的概率提升到 $1 - 1 / \mathop{\text{poly}}\nolimits(n)$。
    - 求出 $R^1(I')$ 的时间复杂度为 $O(m + n \log n)$。
    - 总时间复杂度为 $O((m + n \log n) \log n)$。
    - 如果若干次采样后都没有找到满足条件的 $I$，说明很可能 $H, L$ 的划分不正确，重启算法。
- 至此，我们已经完成了整个算法的设计。
- 论文中对参数的选取给出了 $c = 9$ 和 $c' = 8$ 可以保证算法重启的概率不超过 $2 / n^2$。

最后，我们将前述所有内容整合：

- 总体的思路是，我们有方法（有潜力以平均每条负边快于 $\mathcal O(m)$ 的速度）消除这两类负边：
  - $r$ 遥远负边：复杂度 $O((r + k / r) (m + n \log n))$（这里 $k$ 是负边条数）；
  - $1$ 跳无关负边：复杂度 $O(m + n \log n)$。
- 然后我们需要有方法找到这两类负边，且找到的负边集足够大。
- 对于 $r$ 遥远负边，构造了一类称作负夹心面包的结构：
  - 在间性足够小的图中，找到一个足够大的负夹心面包，就能得到一个足够大的 $r$ 遥远负边集。
  - 具体来说，如果 $\mathop{\text{BW}}\nolimits^\beta(u \rightsquigarrow v) \le n / \tau$，且找到了一个负夹心面包 $(x, U, y)$，则经过一个合适的势函数后， $U$ 就是一个 $\min(\tau, \beta - 1)$ 遥远负边集。
  - 这个势函数在 $O(\beta (m + n \log n))$ 时间内可以计算出来。
- 我们先想办法降低图的间性：
  - 存在一个随机算法，在 $O(\beta \tau (m + n \log n) \log n + \tau^2 n \log^2 n)$ 的时间内，以 $1 - 1 / \mathop{\text{poly}}\nolimits(n)$ 的概率找到一个势函数，使得新图中每对点的 $\beta$ 间性都 $\le n / \tau$。
- 又有，存在一个算法，在期望 $O((m + n \log n) \log n)$ 的时间内，给出如下三种结果之一：
  1. 汇报图中有负环；
  2. 找到一个大小为 $\Omega(k^{1/3})$ 的负夹心面包；
  3. 找到一个大小为 $\Omega(k^{1/3})$ 的 $1$ 跳无关负边集。
- 如果找到了负夹心面包，（且我们已经降低了图的间性，）就能得到一个足够大的 $r$ 遥远负边集。
  - 这一步的时间复杂度为 $O(\beta (m + n \log n))$。
  - 这一步也可能会失败，即图的间性其实不够小，需要重启算法。
- 否则，也能找到一个足够大的 $1$ 跳无关负边集。
- 还需确定参数 $r, \beta, \tau$。显然取 $r = \tau = \beta - 1$ 是不劣的。即 $\tau, \beta$ 都是 $\Theta(r)$ 的。
- 此时最终复杂度为 $O(((k / r) + r^2 \log n) (m + n \log n))$。
- 显然取 $r = \Theta(k^{1/3} \log^{-1/3} n)$ 是最优，为 $O(k^{2/3} (m + n \log n) \log^{1/3} n)$。
- 又这里的 $k$（即消除 $k$ 条 $r$ 遥远负边）其实是当前负边数的 $1 / 3$ 次方，故应写作 $O(k^{2/9} (m + n \log n) \log^{1/3} n)$。
- 每次可以消除 $\Omega(k^{1/3})$ 条负边，重复如上过程 $\Theta(k^{2/3})$ 次，即可消除常数比例的负边。
- 故总复杂度为 $O(n^{8/9} (m + n \log n) \log^{1/3} n)$，或写作 $O(m n^{8/9} \log^{1/3} n + n^{17/9} \log^{4/3} n)$。

注： $m$ 上比 $n$ 少一个 $\log n$ 是因为默认 Dijkstra 使用了 Fibonacci 堆。代码实现中直接用的二叉堆，会多一个 $\log n$。

## 附

**Chernoff–Hoeffding 定理**：

- 设 $X_1, \ldots, X_n$ 独立同分布 Bernoulli 随机变量，且 $\mathop{\text{E}}\nolimits[X_1] = \mu$。
- 则，对 $\varepsilon > 0$：
  1. $\displaystyle \Pr \biggl( \frac{1}{n} \sum X \ge \mu + \varepsilon \biggr) \le \biggl( \Bigl( \frac{\mu}{\mu + \varepsilon} \Bigr)^{\mu + \varepsilon} \Bigl( \frac{1 - \mu}{1 - \mu - \varepsilon} \Bigr)^{1 - \mu - \varepsilon} \biggr)^n$。
  2. $\displaystyle \Pr \biggl( \frac{1}{n} \sum X \le \mu - \varepsilon \biggr) \le \biggl( \Bigl( \frac{\mu}{\mu - \varepsilon} \Bigr)^{\mu - \varepsilon} \Bigl( \frac{1 - \mu}{1 - \mu + \varepsilon} \Bigr)^{1 - \mu + \varepsilon} \biggr)^n$。
- 来自 <https://en.wikipedia.org/wiki/Chernoff_bound#Additive_form_(absolute_error)>。
- 特别地，当取 $\mu + \varepsilon = 1 / 2$（当 $\mu < 1 / 2$）和取 $\mu - \varepsilon = 1 / 2$（当 $\mu > 1 / 2$）时，有：
  1. $\displaystyle \Pr \biggl( \frac{1}{n} \sum X \ge \frac{1}{2} \biggr) \le \biggl( \Bigl( \frac{\mu}{1 / 2} \Bigr)^{1 / 2} \Bigl( \frac{1 - \mu}{1 / 2} \Bigr)^{1 / 2} \biggr)^n = (\sqrt{1 - 4 (1 / 2 - \mu)^2})^n$。
  2. $\displaystyle \Pr \biggl( \frac{1}{n} \sum X \le \frac{1}{2} \biggr) \le \biggl( \Bigl( \frac{\mu}{1 / 2} \Bigr)^{1 / 2} \Bigl( \frac{1 - \mu}{1 / 2} \Bigr)^{1 / 2} \biggr)^n = (\sqrt{1 - 4 (\mu - 1 / 2)^2})^n$。
