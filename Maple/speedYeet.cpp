// // make a speed test to compare the speed of 1000 inserts and 100000 searches
// on
// // unordered_map and map with strings
// #include <algorithm>
// #include <chrono>
// #include <iostream>
// #include <iterator>
// #include <map>
// #include <random>
// #include <string>
// #include <unordered_map>
// #include <vector>

// using namespace std;
// string getRandomString() {
// 	string s = "";
// 	for (int i = 0; i < 10; i++) {
// 		s += (char)(rand() % 60 + 65);
// 	}
// 	return s;
// }
// int main() {
// 	// create a vector of 1000 random strings
// 	vector<string> v;
// 	for (int i = 0; i < 1000; i++) {
// 		v.push_back(getRandomString());
// 	}
// 	// create a map and unordered_map
// 	map<string, int> m;
// 	unordered_map<string, int> um;
// 	// start the timer for the map insert
// 	auto start = chrono::high_resolution_clock::now();
// 	// insert the 1000 random strings into the map
// 	for (int i = 0; i < 1000; i++) {
// 		m.insert(pair<string, int>(v[i], i));
// 	}
// 	// stop the timer for the map insert
// 	auto stop = chrono::high_resolution_clock::now();
// 	auto duration = chrono::duration_cast<chrono::nanoseconds>(stop - start);
// 	cout << "map: " << duration.count() << " microseconds" << endl;
// 	// start the timer for the unordered_map insert
// 	start = chrono::high_resolution_clock::now();
// 	// insert the 1000 random strings into the unordered_map
// 	for (int i = 0; i < 1000; i++) {
// 		um.insert(pair<string, int>(v[i], i));
// 	}
// 	// stop the timer for the unordered_map insert
// 	stop = chrono::high_resolution_clock::now();

// 	// calculate the time it took to insert the 1000 random strings into the
// 	// unordered_map
// 	duration = chrono::duration_cast<chrono::nanoseconds>(stop - start);
// 	cout << "unordered_map: " << duration.count() << " microseconds" << endl;

// 	// create a vector of 100000 random strings
// 	vector<string> v2;
// 	for (int i = 0; i < 100000; i++) {
// 		v2.push_back(getRandomString());
// 	}
// 	// start the timer for the map
// 	start = chrono::high_resolution_clock::now();
// 	// search for the 100000 random strings in the map
// 	for (int i = 0; i < 100000; i++) {
// 		[[maybe_unused]] auto s = m.find(v2[i]);
// 	}
// 	// stop the timer for the map
// 	stop = chrono::high_resolution_clock::now();
// 	// calculate the time it took to search for the 100000 random strings in the
// 	// map
// 	duration = duration_cast<chrono::nanoseconds>(stop - start);
// 	cout << "map: " << duration.count() << " ns" << endl;
// 	// start the timer for the unordered_map
// 	start = chrono::high_resolution_clock::now();
// 	// search for the 100000 random strings in the unordered_map
// 	for (int i = 0; i < 100000; i++) {
// 		[[maybe_unused]] auto s = um.find(v2[i]);
// 	}
// 	// stop the timer for the unordered_map
// 	stop = chrono::high_resolution_clock::now();
// 	// calculate the time it took to search for the 100000 random strings in the
// 	// unordered_map
// 	duration = chrono::duration_cast<chrono::nanoseconds>(stop - start);
// 	cout << "unordered_map: " << duration.count() << " ns" << endl;
// 	return 0;
// }