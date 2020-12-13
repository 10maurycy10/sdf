// undoubtedly bad code
let badcode = (()=>{
	let x=0;
	return ()=>(x+=1);
})();
assert(badcode() != badcode());

// by extention this is bad to
let classybadcode = (x)=>{
	let y=x;
	return {
		getbad: () => y+=1
	}
};
let my_badcode = classybadcode(123);
assert(my_badcode.getbad() != my_badcode.getbad())

// while this is good code
let notbad = (x)=>()=>x;
let my_not_bad = notbad(5)
assert(my_not_bad() == my_not_bad())

// altho you can still make bad code
let K = (x)=>()=>x
let S = (a)=>(b)=>(c)=>a(c)(b(c))
let I = S(K)(K)
let Y = S(K)(K)(S(K(S(S)(S(S(S)(K)))))(K))
let one = Y((x)=>(1/x))
assert(one(3) == one(5))

