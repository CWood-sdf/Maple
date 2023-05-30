export module LinkedList;

#ifndef NULL
#define NULL 0
#endif // !NULL
import <type_traits>;
import <new>;


template<class _Tp1, class _Tp2>
inline typename std::enable_if<
	!std::is_same<_Tp1, _Tp2>::value
	&& !std::is_same<_Tp1, _Tp2*>::value, _Tp1>::type
	getAsPtr(_Tp2&) {
	return _Tp1();
}

template<class _Tp1, class _Tp2>
inline typename std::enable_if<
	std::is_same<_Tp1, _Tp2>::value, _Tp1>::type
	getAsPtr(_Tp2& val) {
	return val;
}

template<class _Tp1, class _Tp2>
inline typename std::enable_if<
	std::is_same<_Tp1, _Tp2*>::value, _Tp1>::type
	getAsPtr(_Tp2& val) {
	return &val;
}



template<class _Tp, class _RefTp, class _PtrTp, class RawTp>
class BasicLinkedList
{


	friend class ListNode;
	friend class Iterator;
	friend class RIterator;

protected:
	class Friend {};
	class ListNode;
	class Iterator;
	class RIterator;
public:
	typedef ListNode Node;
	typedef BasicLinkedList<_Tp, _RefTp, _PtrTp, RawTp> List;
protected:
	static Node* makeEmptyNode() {
		Node* node = (Node*)std::malloc(sizeof(Node));
		node->isExistNode = false;
		return node;
	}
	static Node* emptyNode;
	Node* base = NULL;
	Node* endn = NULL;
	Node* current = NULL;
	bool bound = false;

public:
	/**
	 * @brief calls std::move on the value if true, otherwise just adds it
	 *
	 * @tparam tempVal
	 * @param val the value to be added
	 */
	template<bool tempVal>
	void basicPush(_Tp& val);
	template<>
	void basicPush<true>(_Tp& val) {
		Node* newEnd = new Node(std::move(val));
		if (endn == NULL) {
			base = endn = current = newEnd;
		}
		else {
			endn->next = newEnd;
			newEnd->prev = endn;
			endn = newEnd;

		}
		if (current == NULL) {
			current = endn;
		}
	}
	template<>
	void basicPush<false>(_Tp& val) {
		Node* newEnd = new Node(val);
		if (endn == NULL) {
			base = endn = current = newEnd;
		}
		else {
			endn->next = newEnd;
			newEnd->prev = endn;
			endn = newEnd;

		}
		if (current == NULL) {
			current = endn;
		}
	}
	/**
	 * @brief calls basicPush<false>(val)
	 *
	 * @param val the value to be added
	 */
	void push(_Tp& val) {
		basicPush<false>(val);
	}
	/**
	 * @brief calls basicPush<true>(val)
	 *
	 * @param val the value to be added
	 */
	void movePush(_Tp& val) {
		basicPush<true>(val);
	}


	//Iterators
	Iterator begin() {
		reset();
		return Iterator(base, current, (List*)this);
	}
	Node* end() const volatile {
		return endn;
	}
	Iterator cbegin() const {
		reset();
		return Iterator(base, current, this);
	}
	Node* cend() const {
		return endn;
	}

	RIterator rbegin() {
		resetEnd();
		return RIterator(endn, current, this);
	}
	Node* rend() {
		return base;
	}
	RIterator crbegin() const {
		resetEnd();
		return RIterator(endn, current, this);
	}
	Node* crend() const {
		return base;
	}
protected:
	void constructor(List& llist) {
		if (llist.empty()) return;
		for (_Tp l : llist) {
			push(l);
		}
		current = base;

	}
	void constructor(Node* b) {
		if (b == NULL) {
			return;
		}
		base = new Node(b, endn);
		current = base;
	}
	void constructor(_Tp e1, _Tp e2) {
		pushBack(e1);
		pushBack(e2);
	}
	void constructor(_Tp e) {
		pushBack(e);
	}
	void cconstructor(List& list) {
		constructor(list);
	}
	void constructor(const List& list) {
		if (list.empty()) return;
		//base = new Node(list.base, endn);
		auto* l = (char*)&list;
		l--;
		l++;
		List* nl = (List*)l;
		List& al = *nl;
		for (_Tp l : al) {
			push(l);
		}
		current = base;
	}
	void constructor(const volatile List& list) {
		auto* l = (char*)&list;
		l--;
		l++;
		List* nl = (List*)l;
		List& al = *nl;
		for (_Tp l : al) {
			push(l);
		}
		current = base;
	}
	void constructor(volatile List& list) {
		auto* l = (char*)&list;
		l--;
		l++;
		List* nl = (List*)l;
		List& al = *nl;
		for (_Tp l : al) {
			push(l);
		}
		current = base;
	}
	void constructor(std::initializer_list<_Tp> llist) {
		for (_Tp g : llist) {
			pushBack(g);
		}
	}

	void destructor() {
		if (empty() || bound);
		else {
			current = base;
			while (!empty()) {
				popBase();
			}
		}
		bound = false;
		base = NULL;
		endn = NULL;
		current = NULL;

	}
public:
	/**
	 * @brief Removes the objects ties to the list pointers without deleting the elements
	 *
	 * @warning This WILL NOT delete the elements in the list, USE WITH CAUTION
	 *
	 */
	void dissolve() {

		base = endn = current = NULL;
		bound = false;
	}
	/**
	 * @brief Strange constructor, all the arguments are parameters for the construction of one element's value
	 *
	 * @tparam Arg1
	 * @tparam Arg2
	 * @tparam Args
	 * @param a1
	 * @param a2
	 * @param args
	 */
	template<typename Arg1, typename Arg2, typename ... Args>
	BasicLinkedList(Arg1 a1, Arg2 a2, Args... args) : BasicLinkedList(_Tp(a1, a2, args...)) {

	}
	/**
	 * @brief Construct a new empty Linked List object
	 *
	 */
	BasicLinkedList() {

	}
	/**
	 * @brief Construct a new Basic Linked List object
	 *
	 * @param seed The value of the base of the list
	 */
	BasicLinkedList(_Tp seed) {
		constructor(seed);
	}
	/**
	 * @brief Construct a new Basic Linked List object
	 *
	 * @param llist The list to copy the elements off of
	 */
	BasicLinkedList(std::initializer_list<_Tp> llist) {
		constructor(llist);
	}
	/**
	 * @brief Construct a new Basic Linked List object
	 *
	 * @tparam Args A parameter pack of elements
	 * @param e1 The first element
	 * @param e2 the second element
	 * @param args the rest of the elements
	 */
	template<typename ... Args>
	BasicLinkedList(_Tp e1, _Tp e2, Args ... args) : BasicLinkedList(static_cast<_Tp>(args)...) {
		constructor(e1, e2);
	}
	/**
	 * @brief Construct a new Basic Linked List object
	 *
	 * @param list list to copy
	 */
	BasicLinkedList(BasicLinkedList& list) {
		constructor(list);
	}
	/**
	 * @brief Construct a new Basic Linked List object
	 *
	 * @param llist list to copy
	 */
	BasicLinkedList(const BasicLinkedList& llist) {
		constructor(llist);
	}
	BasicLinkedList(volatile BasicLinkedList& list) {
		constructor(list);
	}
	BasicLinkedList(const volatile BasicLinkedList& llist) {
		constructor(llist);
	}
	/**
	 * @brief Construct a new Basic Linked List object
	 *
	 * @param llist list to move
	 */
	BasicLinkedList(BasicLinkedList&& llist) noexcept {
		constructor(llist);
		llist.destructor();
	}
	/**
	 * @brief Destroy the Basic Linked List object, cleans up the memory if the elements are not owned by another list
	 *
	 */
	~BasicLinkedList() {
		if (!bound) {
			destructor();
		}
		else {
			int g = 0;
			g++;
		}
	}
	/**
	 * @brief Returns the first node that matches the given value
	 *
	 * @param e a value to search for
	 * @return Node& the node that matches the value
	 */
	Node& find(_Tp e) {
		if (empty()) return *emptyNode;
		auto n = base;
		while (n != endn) {
			if (n->value == e) {
				return *n;
			}
			n = n->next;
		}
		if (n->value == e) return *n;
		return *emptyNode;
	}
	/**
	 * @brief Get the Current list element
	 *
	 * @return Node&
	 */
	Node& getCurrent() {
		if (current == NULL) {
			return *emptyNode;
		}
		return *current;
	}
	/**
	 * @brief Get the Base object
	 *
	 * @return Node&
	 */
	Node& getBase() {
		if (base == NULL)
			return *emptyNode;
		return *base;
	}
	/**
	 * @brief Get the End object
	 *
	 * @return Node&
	 */
	Node& getEnd() {
		if (endn == NULL) return *emptyNode;
		return *endn;
	}
	/*List getCopy() {
		List ret = List();
		ret.cconstructor(*this);
		ret.bound = true;
		return ret;
	}*/
	/**
	 * @brief Adds the new list after the current element
	 *
	 * @param llist the list to add
	 * @param moveToEnd moves the current element to the end of the list added
	 */
	void addAfter(List& llist, bool moveToEnd = false) {
		List newList = List(llist);
		if (current == NULL) {
			current = base;
		}
		if (newList.empty()) return;
		if (empty()) {
			joinTo(newList);

		}
		else if (current->next == NULL) {
			current->next = newList.base;
			newList.base->prev = current;
			endn = newList.endn;
			if (moveToEnd)
				current = endn;
		}
		else {
			Node* next = current->next;
			next->prev = NULL;
			current->next = NULL;
			current->next = newList.base;
			current->next->prev = current;
			next->prev = newList.endn;
			next->prev->next = next;
			if (moveToEnd)
				current = next;
		}
		newList.dissolve();

	}
	/**
	 * @brief Adds an element after the current element
	 *
	 * @param llist the element to add
	 * @param moveToEnd moves the current element to the end of the list added
	 */
	void addAfter(_Tp llist, bool moveToEnd = false) {
		Node* newList = new Node(llist);
		if (current == NULL) {
			current = base;
		}
		if (empty()) {
			base = current = endn = newList;

		}
		else if (current->next == NULL) {
			current->next = newList;
			newList->prev = current;
			endn = newList;
			if (moveToEnd)
				current = endn;
		}
		else {
			Node* next = current->next;
			next->prev = NULL;
			current->next = NULL;
			current->next = newList;
			current->next->prev = current;
			next->prev = newList;
			next->prev->next = next;
			if (moveToEnd)
				current = next;
		}

	}
	/**
	 * @brief Adds the new list before the current element
	 *
	 * @param llist the list to add
	 * @param moveToBeg moves the current element to the beginning of the list added
	 */
	void addBefore(List& llist, bool moveToBeg = false) {
		List newList = List(llist);
		if (current == NULL) {
			current = base;
		}
		if (newList.empty()) return;
		if (empty()) {
			joinTo(newList);

		}
		else if (current->prev == NULL) {
			current->prev = newList.endn;
			newList.endn->next = current;
			base = newList.base;
			if (moveToBeg)
				current = base;
		}
		else {
			Node* prev = current->prev;
			prev->next = NULL;
			current->prev = NULL;
			current->prev = newList.endn;
			current->prev->next = current;
			prev->next = newList.base;
			prev->next->prev = prev;
			if (moveToBeg)
				current = prev;
		}
		newList.dissolve();

	}
	/**
	 * @brief Adds an element before the current element
	 *
	 * @param llist the element to add
	 * @param moveToBeg moves the current element to the beginning of the list added
	 */
	void addBefore(_Tp llist, bool moveToBeg = false) {
		Node* newList = new Node(llist);
		if (current == NULL) {
			current = base;
		}
		if (empty()) {
			base = endn = current = newList;

		}
		else if (current->prev == NULL) {
			current->prev = newList;
			newList->next = current;
			base = newList;
			if (moveToBeg)
				current = base;
		}
		else {
			Node* prev = current->prev;
			prev->next = NULL;
			current->prev = NULL;
			current->prev = newList;
			current->prev->next = current;
			prev->next = newList;
			prev->next->prev = prev;
			if (moveToBeg)
				current = prev;
		}

	}

	/**
	 * @brief constructs a new element from the arguments and adds it to the end
	 *
	 * @tparam Arg1
	 * @tparam Arg2
	 * @tparam Args
	 * @param a1
	 * @param a2
	 * @param args
	 */
	template<typename Arg1, typename Arg2, typename ... Args>
	void pushBack(Arg1 a1, Arg2 a2, Args... args) {
		pushBack(_Tp(a1, a2, args...));
	}
	/**
	 * @brief adds the element to the list
	 *
	 * @param val
	 */
	void pushBack(_Tp& val) {
		push(val);
	}
	/**
	 * @brief constructs a new element from the arguments and adds it to the beginning, only enabled if the list is not a pointer-ref
	 *
	 * @tparam r
	 * @param val
	 * @return std::enable_if<
	 * !std::is_same<_Tp, r*&>::value, void>::type
	 */
	template<class r = RawTp>
	inline typename std::enable_if<
		!std::is_same<_Tp, r*&>::value, void>::type
		pushBack(_Tp&& val) {
		movePush(val);

	}
	/**
	 * @brief Adds the element to the end of the list
	 *
	 * @param n the node to add
	 */
	void pushBack(Node& n) {
		pushBack(n.val);
	}
	/**
	 * @brief Concats the list onto the end of this list
	 *
	 * @param list the list to add
	 */
	void pushBack(List& list) {
		if (list.empty())
			return;
		List addList = List(list);
		if (empty()) {
			joinTo(addList);
		}
		else {
			endn->next = addList.base;
			addList.base->prev = endn;
			endn = addList.endn;
		}
		addList.dissolve();
	}
	/**
	 * @brief constructs a new element from the arguments and adds it to the beginning
	 *
	 * @tparam Arg1
	 * @tparam Arg2
	 * @tparam Args
	 * @param a1
	 * @param a2
	 * @param args
	 */
	template<typename Arg1, typename Arg2, typename ... Args>
	void pushBase(Arg1 a1, Arg2 a2, Args... args) {
		pushBase(_Tp(a1, a2, args...));
	}
	/**
	 * @brief adds the element to the base of the list
	 *
	 * @param val the value to add
	 */
	void pushBase(_Tp& val) {
		Node* newBase = new Node(val);
		if (endn == NULL) {
			base = endn = current = newBase;
		}
		else {
			base->prev = newBase;
			newBase->next = base;
			base = newBase;

		}
		if (current == NULL) {
			current = base;
		}
	}
	/**
	 * @brief constructs a new element from the arguments and adds it to the beginning, only enabled if the list is not a pointer-ref
	 *
	 * @tparam r
	 * @param val
	 * @return std::enable_if<
	 * !std::is_same<_Tp, r*&>::value, void>::type
	 */
	template<class r = RawTp>
	inline typename std::enable_if<
		!std::is_same<_Tp, r*&>::value, void>::type
		pushBase(_Tp&& val) {
		pushBase(val);

	}
	/**
	 * @brief Adds the element to the beginning of the list
	 *
	 * @param n the node to add
	 */
	void pushBase(Node& n) {
		pushBase(n.val);
	}
	/**
	 * @brief Concats the list onto the beginning of this list
	 *
	 * @param list the list to add
	 */
	void pushBase(List& list) {
		if (list.empty())
			return;
		List addList = List(list);
		if (empty()) {
			joinTo(addList);
		}
		else {
			base->prev = addList.endn;
			addList.base->next = base;
			base = addList.base;
		}
		addList.dissolve();
	}
	/**
	 * @brief removes the end node
	 *
	 */
	void popEnd() {
		if (endn == NULL) return;
		if (endn->prev == NULL) {
			delete endn;
			dissolve();
		}
		else {
			Node* oldEnd = endn;
			endn = endn->prev;
			if (current == oldEnd) {
				current = endn;
			}
			oldEnd->dissolve();
			delete oldEnd;
			endn->next = NULL;
		}
	}
	/**
	 * @brief removes the base node
	 *
	 */
	void popBase() {
		// cout << "P" << base << endl;
		// cout << "Pn" << base->next << endl;
		if (base == NULL) return;
		if (base->next == NULL) {
			Node* del = base;

			delete del;
			dissolve();
		}
		else {
			Node* oldBase = base;
			base = base->next;
			oldBase->dissolve();
			if (current == oldBase) {
				current = base;
			}
			delete oldBase;
			base->prev = NULL;
		}
	}
	/**
	 * @brief Reference (kinda) binds the list to the given list
	 *
	 * @param llist
	 */
	void joinTo(List& llist) {
		destructor();
		bound = true;
		endn = llist.endn;
		base = llist.base;
		current = llist.current;
	}
	/**
	 * @brief Removes the current node, shifts the current node to the previous node
	 *
	 */
	void popCurrent() {
		if (current == NULL) return;
		if (current->next == NULL && current->prev == NULL) {
			current->dissolve();
			delete current;
			dissolve();
		}
		else if (current == endn) {
			popEnd();
			current = endn;
		}
		else if (current == base) {
			popBase();
			current = base;
		}
		else {
			Node* next = current->next;
			Node* prev = current->prev;
			current->dissolve();
			next->prev = prev;
			prev->next = next;
			delete current;
			current = prev;
		}
	}
	/**
	 * @brief Removes the current node, shifts the current node to the next node
	 *
	 */
	void popCurrentNext() {
		if (current == NULL) return;
		if (current->next == NULL && current->prev == NULL) {
			current->dissolve();
			delete current;
			dissolve();
		}
		else if (current == endn) popEnd();
		else if (current == base) popBase();
		else {
			Node* next = current->next;
			Node* prev = current->prev;
			current->dissolve();
			next->prev = prev;
			prev->next = next;
			delete current;
			current = next;
		}
	}
	/**
	 * @brief Returns true if the list is empty
	 *
	 * @return true
	 * @return false
	 */
	bool empty() const {
		return base == NULL;
	}
	/**
	 * @brief Empty the list
	 *
	 */
	void clear() {
		destructor();
		dissolve();
	}
	/**
	 * @brief Set the Current object to the given node
	 *
	 * @param n a node
	 */
	void setCurrent(Node& n) {
		//Check if the node is in the list
		Node* c = base;
		while (c != NULL) {
			if (c == &n) {
				current = &n;
				return;
			}
			c = c->next;
		}
	}
	/**
	 * @brief Moves the current node closer to the base
	 *
	 */
	void moveCurrentLeft() {
		if (!empty() && current != base) {
			current = current->prev;
		}
	}
	/**
	 * @brief Moves the current node closer to the end
	 *
	 */
	void moveCurrentRight() {
		if (!empty() && current != endn) {
			current = current->next;
		}
	}
	List& operator= (List& llist) {
		destructor();
		constructor(llist);
		bound = false;
		return *this;
	}
	List& operator= (const List& llist) {
		destructor();
		bound = false;
		constructor(llist);
		return *this;
	}
	List& operator= (const volatile List& llist) {
		destructor();
		bound = false;
		constructor(llist);
		bound = false;
		return *this;
	}
	List& operator= (List&& llist) noexcept {
		destructor();
		bound = false;
		constructor(llist);
		bound = false;

		llist.destructor();
		return *this;
	}
private:
	int size(Node* n) {
		if (n->next == NULL) {
			return 0;
		}
		else {
			return size(n->next) + 1;
		}
	}
	bool size(Node* n, int& cSize, int wantSize) {
		if (n->next != NULL) {
			cSize++;
			if (wantSize < cSize) {
				return false;
			}
			return size(n->next, cSize, wantSize);
		}
		else {
			if (wantSize - 1 == cSize) {
				return true;
			}
			return false;
		}
	}
	int iters = 0;
public:
	/**
	 * @brief Returns the size of the list
	 *
	 * @return int
	 */
	int size() {
		if (empty()) return 0;
		return size(base) + 1;
	}
	/**
	 * @brief Returns true if the list is of the given size
	 *
	 * @param wantSize the size to check
	 * @return true
	 * @return false
	 */
	bool size(int wantSize) {
		int cSize = 0;
		if (empty()) {
			if (wantSize == 0) {
				return true;
			}
			return false;
		}
		return size(base, cSize, wantSize);
	}
	/**
	 * @brief Unbinds the list
	 *
	 */
	void unbind() {
		bound = false;
	}
	/**
	 * @brief Calls moveCurrentRight
	 *
	 * @return Node&
	 */
	Node& operator++() const volatile {
		if (current == endn) {
			return *emptyNode;
		}
		else if (current == NULL) {
			char* l = (char*)this;
			List* nl = (List*)l;
			nl->current = base->next;
			return *current;
		}
		else {
			char* l = (char*)this;
			List* nl = (List*)l;
			nl->current = current->next;
			return *current;
		}
	}
	/**
	 * @brief Calls moveCurrentLeft
	 *
	 * @return Node&
	 */
	Node& operator--() {
		if (current == base) {
			return *emptyNode;
		}
		else if (current == NULL) {

			current = endn->prev;
			return *current;
		}
		else {
			current = current->prev;
			return *current;
		}
	}
private:
	bool comp(Node& l1, Node& l2)  const volatile {
		if (l1.val == l2.val) {
			if (l1.getNext().exists()) {
				if (l2.getNext().exists()) {
					return comp(l1.getNext(), l2.getNext());
				}
				else {
					//If list sizes don't match
					return false;
				}
			}
			else if (!l2.getNext().exists()) {
				return true;
			}
		}
		//if list vals don't match
		return false;
	}
public:
	/**
	 * @brief Compares two lists
	 *
	 * @param l
	 * @return true
	 * @return false
	 */
	bool operator==(List& l) {
		if ((empty() && l.empty()) || &l == (List*)this) {
			return true;
		}
		return comp(getBase(), l.getBase());
	}
	/**
	 * @brief Compares two lists
	 *
	 * @param l
	 * @return true
	 * @return false
	 */
	bool operator==(List&& l) {
		return operator==(l);
	}
	/**
	 * @brief Compares two lists
	 *
	 * @param l
	 * @return true
	 * @return false
	 */
	bool operator!=(List& l) {
		return !operator==(l);
	}
	/**
	 * @brief Compares two lists
	 *
	 * @param l
	 * @return true
	 * @return false
	 */
	bool operator!=(List&& l) {
		return !operator==(l);
	}
	/**
	 * @brief Moves the current node to the base
	 *
	 */
	void reset() const volatile {
		//current = base;

		char* l = (char*)this;
		l--;
		l++;
		List* nl = (List*)l;
		nl->current = nl->base;

	}
	/**
	 * @brief Moves the current node to the end
	 *
	 */
	void resetEnd() const volatile {
		//A little pointer workaround that allows me to change current
		char* l = (char*)this;
		l--;
		l++;
		List* nl = (List*)l;
		nl->current = nl->endn;
	}

};
template<class _Tp, class _RefTp, class _PtrTp, class RawTp>
//The nodes
class BasicLinkedList<_Tp, _RefTp, _PtrTp, RawTp>::ListNode {
	friend class BasicLinkedList<_Tp, _RefTp, _PtrTp, RawTp>;
	friend class BasicLinkedList<_Tp, _RefTp, _PtrTp, RawTp>::Iterator;
	friend void sdfsdfr();
public:
	typedef ListNode Node;
protected:

	Node* next = NULL;
	Node* prev = NULL;
	//bool deletable = true;
	bool isExistNode = true;
	
	/*void preventDel() {
		deletable = false;
	}*/
public:
	_Tp val;
	ListNode(Node* base, Node*& endn) : val(base->val) {
		if (base->next != NULL) {
			next = new Node(base->next, endn);
			next->prev = this;
		}
		else {
			endn = this;
		}
	}
	ListNode(_Tp& val) : val(val) {

	}
	ListNode(_Tp&& val) : val(std::move(val)) {

	}
	ListNode(ListNode& n) : val(n.val) {
		isExistNode = n.isExistNode;
		//deletable = n.deletable;
	}
	ListNode(const ListNode& n) : val(n.val) {
		isExistNode = n.isExistNode;
		//deletable = n.deletable;
	}
	~ListNode() {

	}

	//void allowDel() {
	//	//deletable = true;
	//}
	void dissolve() {
		next = prev = NULL;
	}
	/**
	 * @brief Returns the next node
	 *
	 * @return Node&
	 */
	Node& getNext() const volatile {
		if (next == NULL) {
			return *emptyNode;
		}
		return *next;
	}
	/**
	 * @brief Returns the previous node
	 *
	 * @return Node&
	 */
	Node& getPrev() const volatile {
		if (prev == NULL) {
			return *emptyNode;
		}
		return *prev;
	}
	/**
	 * @brief Returns true if the node exists
	 *
	 * @return true
	 * @return false
	 */
	bool exists() const volatile {
		return isExistNode;
	}
	/**
	 * @brief Returns true if the node does not exist
	 *
	 * @return true
	 * @return false
	 */
	bool notexists() const volatile {
		return !exists();
	}
	bool operator==(Node& n) const volatile {
		return this == &n;
	}




	_PtrTp operator->() {
		return getAsPtr<_PtrTp, _Tp>(val);
	}
	operator _RefTp () {
		return val;
	}
	Node& operator=(_Tp& t) {
		val = t;
		return *this;
	}
	template<class r = RawTp>
	inline typename std::enable_if<
		!std::is_same<_Tp, r*&>::value, Node&>::type
		operator=(_Tp&& t) {
		val = t;
		return *this;
	}
	Node& operator=(Node& t) {
		val = t.val;
		return *this;
	}
};
template<class _Tp, class _RefTp, class _PtrTp, class RawTp>
//The setup for the iterators
class BasicLinkedList<_Tp, _RefTp, _PtrTp, RawTp>::Iterator {
protected:
	friend void sdfsdfr();
	typedef ListNode Node;
	typedef BasicLinkedList<_Tp, _RefTp, _PtrTp, RawTp> List;
	Node* node;
	Node*& current;
	List* list;
	bool hit = false;
public:
	Iterator(Node* n, Node*& c, List* l) : current(c) {
		list = l;
		node = n;
	}
	Iterator(Node* n, Node*& c, const List* l) : current(c) {
		list = l;
		node = n;
	}
	Iterator(Node* n, Node*& c, volatile List* l) : current(c) {
		list = l;
		node = n;
	}
	Iterator(Node* n, Node*& c, const volatile List* l) : current(c) {
		list = l;
		node = n;
	}
	~Iterator() {

	}
	void operator++() {
		if (list->empty()) {
			hit = true;
			return;
		}
		if (node != current)
			node = current;
		node = node->next;
		if (node != NULL)
			current = node;
		else {
			hit = true;
			list->reset();
		}
	}
	bool operator!=(Node* i) {
		if (list->empty()) return false;
		if (hit) return false;
		if (i == node) {
			if (hit) {
				return false;
			}
			else {
				hit = true;
				return true;
			}
		}
		return i != node;
	}


	//
	_RefTp operator*() {
		return node->val;
	}
};
template<class _Tp, class _RefTp, class _PtrTp, class RawTp>
class BasicLinkedList<_Tp, _RefTp, _PtrTp, RawTp>::RIterator {
protected:
	friend void sdfsdfr();
	typedef ListNode Node;
	typedef BasicLinkedList<_Tp, _RefTp, _PtrTp, RawTp> List;
	Node* node;
	Node emptyNode = Node();
	Node*& current;
	List* list;
	bool hit = false;
public:
	RIterator(Node* n, Node*& c, List* l) : current(c) {
		list = l;
		node = n;
	}
	~RIterator() {

	}

	//
	_RefTp operator*() {
		return node->val;
	}
	void operator--() {
		if (node != current)
			node = current;
		node = node->prev;
		if (node != NULL)
			current = node;
		else {
			hit = true;
			list->reset();
		}
	}
	bool operator!=(Node* i) {
		if (list->empty()) return false;
		if (hit) return false;
		if (i == node) {
			return true;
		}
		return i != node;
	}
};

template<class T, class T2, class T3, class T4>
BasicLinkedList<T, T2, T3, T4>::Node* BasicLinkedList<T, T2, T3, T4>::emptyNode = BasicLinkedList<T, T2, T3, T4>::makeEmptyNode();


export namespace std {
	export template<class _Tp>
	class LinkedList : public BasicLinkedList<_Tp, _Tp&, _Tp*, _Tp> {
	public:
		typedef BasicLinkedList<_Tp, _Tp&, _Tp*, _Tp> BaseList;
		template<typename Arg1, typename Arg2, typename ... Args>
		LinkedList(Arg1 a1, Arg2 a2, Args... args) : BaseList(_Tp(a1, a2, args...)) {

		}
		LinkedList() : BaseList() {

		}
		LinkedList(_Tp seed) : BaseList(seed) {

		}
		LinkedList(std::initializer_list<_Tp> llist) : BaseList(llist) {

		}
		template<typename ... Args>
		LinkedList(_Tp e1, _Tp e2, Args ... args) : BaseList(static_cast<_Tp>(args)...) {
			constructor(e1, e2);
		}
		LinkedList(LinkedList& list) : BaseList(list) {

		}
		LinkedList(const LinkedList& llist) : BaseList(llist) {

		}
		LinkedList(volatile LinkedList& list) : BaseList(list) {

		}
		LinkedList(const volatile LinkedList& llist) : BaseList(llist) {

		}
	};
	export template<class ptref>
	class LinkedList<ptref*&> : public BasicLinkedList<ptref*&, ptref*&, ptref*, ptref> {
		typedef ptref*& _Tp;
	public:
		typedef BasicLinkedList<ptref*&, ptref*, ptref*, ptref> BaseList;
		template<typename Arg1, typename Arg2, typename ... Args>
		LinkedList(Arg1 a1, Arg2 a2, Args... args) : BaseList(_Tp(a1, a2, args...)) {

		}
		LinkedList() {

		}
		LinkedList(_Tp seed) : BaseList(seed) {

		}
		LinkedList(std::initializer_list<_Tp> llist) : BaseList(llist) {

		}
		template<typename ... Args>
		LinkedList(_Tp e1, _Tp e2, Args ... args) : BaseList(static_cast<_Tp>(args)...) {
			constructor(e1, e2);
		}
		LinkedList(LinkedList& list) : BaseList(list) {

		}
		LinkedList(const LinkedList& llist) : BaseList(llist) {

		}
		LinkedList(volatile LinkedList& list) : BaseList(list) {

		}
		LinkedList(const volatile LinkedList& llist) : BaseList(llist) {

		}
	};
	export template<class pt>
	class LinkedList<pt*> : public BasicLinkedList<pt*, pt*, pt*, pt> {
		typedef pt* _Tp;
		typedef BasicLinkedList<pt*, pt*, pt*, pt> BaseList;
	public:
		template<typename Arg1, typename Arg2, typename ... Args>
		LinkedList(Arg1 a1, Arg2 a2, Args... args) : BaseList(_Tp(a1, a2, args...)) {

		}
		LinkedList() : BaseList() {

		}
		LinkedList(_Tp seed) : BaseList(seed) {

		}
		LinkedList(std::initializer_list<_Tp> llist) : BaseList(llist) {

		}
		template<typename ... Args>
		LinkedList(_Tp e1, _Tp e2, Args ... args) : BaseList(static_cast<_Tp>(args)...) {
			constructor(e1, e2);
		}
		LinkedList(LinkedList& list) : BaseList(list) {

		}
		LinkedList(const LinkedList& llist) : BaseList(llist) {

		}
		LinkedList(volatile LinkedList& list) : BaseList(list) {

		}
		LinkedList(const volatile LinkedList& llist) : BaseList(llist) {

		}
	};


}
