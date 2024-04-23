package id.ac.ui.cs.advprog.heymartmanagement.controller;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.*;

import java.util.List;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import id.ac.ui.cs.advprog.heymartmanagement.model.Product;
@Controller
@RequestMapping("/product")
public class ProductController {
    @GetMapping("/create")
    public String createProductPage(Model model) {

        Product product = new Product("1", "product 1", 50000, 5);
        model.addAttribute("product", product);
        return "createProduct";
    }

}

