//ulimit -s unlimited
#include <bits/stdc++.h>
using namespace std;
#define ll long long
#define vl vector<ll>
#define vi vector<int>
#define vb vector<bool>
#define pql priority_queue<long long>
#define pqp priority_queue<pair<ll,ll> >
#define pqs priority_queue<ll,vl,greater<ll> >
#define md ((ll) 1e9+7)
#include <ext/pb_ds/assoc_container.hpp>
#include <ext/pb_ds/tree_policy.hpp>
using namespace __gnu_pbds;

typedef tree<int, null_type, std::less<int>, rb_tree_tag, tree_order_statistics_node_update> ordered_set;


// void setio(string name){
//     freopen((name+".txt").c_str(),"r",stdin);
//     freopen((name+".out").c_str(),"w",stdout);
// }

void solve(){
    std::ifstream infile("random_numbers.txt");
    if (!infile) {
        std::cerr << "Could not open the file!" << std::endl;
        return;
    }

    vector<int> numbers;
    int number;
    while (infile >> number) {
        numbers.push_back(number);
    }

    infile.close();
    ordered_set s;
    auto start = std::chrono::high_resolution_clock::now();
    for(auto c:numbers){
        s.insert(c);
    }
    auto end = std::chrono::high_resolution_clock::now();

    std::chrono::duration<double> duration = end - start;
    std::cout << "Time taken to insert in AVL tree by c++ (gnu pbds)" << duration.count() << " seconds" << std::endl;

    // Output the vector size to verify the result
    std::cout << "Number of integers read: " << numbers.size() << std::endl;
}

int main(){
    string s = "random_numbers";
    //setio(s);
    // ios::sync_with_stdio(0);
    // cin.tie(0);cout.tie(0);
    int t;
    //cin >> t;
    t = 1;
    while(t--)
        solve();
    return 0;
}