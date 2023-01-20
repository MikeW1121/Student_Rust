#include <string>
#include "studentRoll.h"

StudentRoll::StudentRoll() {
  head = tail = NULL;
}

void StudentRoll::insertAtTail(const Student &s) {
  Node* temp = new Node; 
  temp->s = new Student(s);
  temp->next = NULL; 
  if(!head){
    head = tail = temp;
  }else{
    tail-> next = temp; 
    tail = temp; 
  }
}

std::string StudentRoll::toString() const {
  std::string result = "["; 
  Node* student = head; 
  while(student){
    result.append(student->s->toString());
    result += "," ;
    student = student -> next; 
  }
  if(result.size() > 1){
      result.at(result.size()-1) = ']';
  }else{
    result.append("]");
  }
  return result;
}

StudentRoll::StudentRoll(const StudentRoll &orig) {
  // STUB
  head = tail = NULL;
  Node* c = orig.head; 
  if(!orig.head){
    head = tail = NULL; 
  }else{
    while(c){
      insertAtTail(*(c->s)); 
      c = c->next; 
    }
  }
}

StudentRoll::~StudentRoll() {
  // STUB
  Node* temp = head; 
  Node* p = head; 
  while(p){
    p = p->next; 
    delete temp->s; 
    delete temp; 
    temp = p; 
  }
}

StudentRoll & StudentRoll::operator =(const StudentRoll &right ) {
  // The next two lines are standard, and you should keep them.
  // They avoid problems with self-assignment where you might free up 
  // memory before you copy from it.  (e.g. x = x)

  if (&right == this) 
    return (*this);

  // TODO... Here is where there is code missing that you need to 
  // fill in...
  Node* temp = head; 
  Node* p = head; 
  while(p){
    p = p->next; 
    delete temp->s; 
    delete temp; 
    temp = p; 
  }
  head = tail = NULL;
  Node* c = right.head; 
  if(!right.head){
    head = tail = NULL; 
  }else{
    while(c){
      insertAtTail(*(c->s)); 
      c = c->next; 
    }
  }

  // KEEP THE CODE BELOW THIS LINE
  // Overloaded = should end with this line, despite what the textbook says.
  return (*this); 
  
}





